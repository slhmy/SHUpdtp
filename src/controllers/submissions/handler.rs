use crate::database::{db_connection, Pool};
use server_core::errors::ServiceError;
use crate::judge_actor::JudgeActorAddr;
use crate::models::users::LoggedUser;
use crate::services::submission;
use actix_web::{get, post, put, web, HttpResponse};
use uuid::Uuid;

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
    judge_actor: web::Data<JudgeActorAddr>,
) -> Result<HttpResponse, ServiceError> {
    info!("{:?}", logged_user.0);
    if logged_user.0.is_none() {
        return Err(ServiceError::Unauthorized);
    }

    let res = web::block(move || {
        submission::create(
            body.region.clone(),
            body.problem_id,
            logged_user.0.unwrap().id,
            body.src.clone(),
            body.language.clone(),
            pool,
            judge_actor,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[get("/{id}")]
pub async fn get(
    web::Path(submission_id): web::Path<Uuid>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if logged_user.0.is_none() {
        return Err(ServiceError::Unauthorized);
    }
    let cur_user = logged_user.0.unwrap();

    let conn = &db_connection(&pool)?;

    use crate::schema::submissions as submissions_schema;
    use diesel::prelude::*;

    let user_id: i32 = submissions_schema::table
        .filter(submissions_schema::id.eq(submission_id))
        .select(submissions_schema::user_id)
        .first(conn)?;

    if cur_user.id != user_id && cur_user.role != "sup" && cur_user.role != "admin" {
        let hint = "No permission.".to_string();
        return Err(ServiceError::BadRequest(hint));
    }

    let res = web::block(move || submission::get(submission_id, pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct GetSubmissionListParams {
    region_filter: Option<String>,
    problem_id_filter: Option<i32>,
    user_id_filter: Option<i32>,
    limit: i32,
    offset: i32,
}

#[get("")]
pub async fn get_list(
    query: web::Query<GetSubmissionListParams>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if logged_user.0.is_none() {
        return Err(ServiceError::Unauthorized);
    }

    let res = web::block(move || {
        submission::get_list(
            query.region_filter.clone(),
            query.problem_id_filter.clone(),
            query.user_id_filter.clone(),
            query.limit,
            query.offset,
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
