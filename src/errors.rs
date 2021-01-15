use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::Error as DBError;
use std::io::Error as IOError;
use actix_web::error::BlockingError;
use std::convert::From;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unable to connect to DB")]
    UnableToConnectToDb,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::UnableToConnectToDb => HttpResponse::InternalServerError()
                .json("Unable to connect to DB, Please try later"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

// we can return early in our handlers if UUID is not valid
// and provide a custom message
impl From<uuid::parser::ParseError> for ServiceError {
    fn from(_: uuid::parser::ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(_kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();
                ServiceError::BadRequest(message)
            }
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<IOError> for ServiceError {
    fn from(error: IOError) -> ServiceError {
        match error.kind() {
            std::io::ErrorKind::NotFound => {
                let message = "An entity was not found, often a file.".to_string();
                ServiceError::BadRequest(message)
            },
            _ => {
                let message = "Something went wrong with file analysis, please check your format.".to_string();
                ServiceError::BadRequest(message)
            },
        }
    }
}

impl From<BlockingError<ServiceError>> for ServiceError {
    fn from(error: BlockingError<ServiceError>) -> ServiceError {
        match error {
            // If not canceled, then return the raw error.
            BlockingError::Error(e) => e,
            BlockingError::Canceled => ServiceError::InternalServerError,
        }
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;