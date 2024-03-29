use actix_web::error::BlockingError;
use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::Error as DBError;
use std::convert::From;
use std::io::Error as IOError;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Internal Server Error: {0}")]
    InternalServerErrorWithHint(String),

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unauthorized: {0}")]
    UnauthorizedWithHint(String),

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
            ServiceError::InternalServerErrorWithHint(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ServiceError::UnableToConnectToDb => HttpResponse::InternalServerError()
                .json("Unable to connect to DB, Please try later"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::UnauthorizedWithHint(ref message) => {
                HttpResponse::Unauthorized().json(message)
            }
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

// we can return early in our handlers if UUID is not valid
// and provide a custom message
impl From<uuid::Error> for ServiceError {
    fn from(_: uuid::Error) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        let message = format!("{:?}", error);
        ServiceError::InternalServerErrorWithHint(message)
    }
}

impl From<IOError> for ServiceError {
    fn from(error: IOError) -> ServiceError {
        match error.kind() {
            std::io::ErrorKind::NotFound => {
                let message = "An entity was not found, often a file.".to_string();
                ServiceError::BadRequest(message)
            }
            _ => {
                let message = "Something went wrong with file analysis, please check your format."
                    .to_string();
                ServiceError::BadRequest(message)
            }
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
