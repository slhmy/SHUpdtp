use actix_web::{web, HttpResponse, get, post, put};
use crate::errors::ServiceError;
use crate::database::Pool;
use crate::services::submission;
use crate::models::users::LoggedUser;

#[derive(Deserialize)]
pub struct CreateSubmissionBody {
    region: Option<String>,
    problem_id: i32,
    src: String,
    language: String,
}

#[post("")]
pub async fn create(
    body: web::Json<CreateSubmissionBody>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
    info!("{:?}", logged_user.0);
    if logged_user.0.is_none() { return Err(ServiceError::Unauthorized); }

    let res = web::block(move || submission::create(
        body.region.clone(),
        body.problem_id,
        logged_user.0.unwrap().id,
        body.src.clone(),
        body.language.clone(),
        pool
    )).await.map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}