use actix_web::{web, HttpResponse, post, get};
use crate::errors::ServiceError;
use crate::database::Pool;
use crate::services::user;

#[derive(Deserialize)]
pub struct RegisterBody {
    name: String,
    password: Option<String>,
    mobile: Option<String>,
    role: String,
}

#[post("")]
pub async fn create(
    body: web::Json<RegisterBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || user::create(
        body.name.clone(),
        body.password.clone(),
        body.mobile.clone(),
        body.role.clone(),
        pool
    )).await.map_err(|e| {
        eprintln!("{}", e);
        ServiceError::InternalServerError
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[get("/{id}")]
pub async fn get(
    web::Path(id): web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || user::get(
        id,
        pool
    )).await.map_err(|e| {
        eprintln!("{}", e);
        ServiceError::InternalServerError
    })?;

    Ok(HttpResponse::Ok().json(&res))
}