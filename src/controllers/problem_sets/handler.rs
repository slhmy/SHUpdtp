use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::users::LoggedUser;
use crate::services::problem_set;
use crate::services::region;
use actix_web::{delete, get, post, put, web, HttpResponse};

#[derive(Deserialize)]
pub struct CreateProblemSetBody {
    region: String,
    name: String,
    introduction: Option<String>,
}

#[post("")]
pub async fn create(
    body: web::Json<CreateProblemSetBody>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
    info!("{:?}", logged_user.0);
    if logged_user.0.is_none() {
        return Err(ServiceError::Unauthorized);
    }
    let cur_user = logged_user.0.unwrap();
    if cur_user.role != "sup" && cur_user.role != "admin" {
        let hint = "No permission.".to_string();
        return Err(ServiceError::BadRequest(hint));
    }

    let res = web::block(move || {
        problem_set::create(
            body.region.clone(),
            body.name.clone(),
            body.introduction.clone(),
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct InsertToProblemSetBody {
    problem_ids: Vec<i32>,
}

#[post("/{region}")]
pub async fn insert_problems(
    web::Path(region): web::Path<String>,
    body: web::Json<InsertToProblemSetBody>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
    info!("{:?}", logged_user.0);
    if logged_user.0.is_none() {
        return Err(ServiceError::Unauthorized);
    }
    let cur_user = logged_user.0.unwrap();
    if cur_user.role != "sup" && cur_user.role != "admin" {
        let hint = "No permission.".to_string();
        return Err(ServiceError::BadRequest(hint));
    }

    let res = web::block(move || {
        region::insert_problems(
            region,
            body.problem_ids.clone(),
            None,
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}
