use crate::auth::region::*;
use crate::database::{Pool, SyncMongo};
use crate::errors::ServiceError;
use crate::judge_actor::JudgeActorAddr;
use crate::models::users::LoggedUser;
use crate::services::region;
use actix_web::{delete, get, post, put, web, HttpResponse};

#[derive(Deserialize)]
pub struct GetRegionListParams {
    self_type: Option<String>,
    limit: i32,
    offset: i32,
}

#[get("")]
pub async fn get_list(
    query: web::Query<GetRegionListParams>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        region::get_list(query.self_type.clone(), query.limit, query.offset, pool)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct InsertProblemToRegionBody {
    problem_ids: Vec<i32>,
}

#[post("/{region}")]
pub async fn insert_problems(
    web::Path(region): web::Path<String>,
    body: web::Json<InsertProblemToRegionBody>,
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

    let res =
        web::block(move || region::insert_problems(region, body.problem_ids.clone(), None, pool))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                e
            })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct GetLinkedProblemColumnParams {
    inner_id_filter: Option<i32>,
    problem_id_filter: Option<i32>,
    title_filter: Option<String>,
    tag_filter: Option<Vec<String>>,
    difficulty_filter: Option<String>,
    inner_id_order: Option<bool>,
    problem_id_order: Option<bool>,
    difficulty_order: Option<bool>,
    limit: i32,
    offset: i32,
}

#[get("/{region}")]
pub async fn get_linked_problem_column_list(
    web::Path(region): web::Path<String>,
    query: web::Query<GetLinkedProblemColumnParams>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser,
    mongodb_database: web::Data<SyncMongo>,
) -> Result<HttpResponse, ServiceError> {
    check_view_right(pool.clone(), logged_user.clone(), region.clone())?;

    let res = web::block(move || {
        region::get_linked_problem_column_list(
            region,
            query.inner_id_filter,
            query.problem_id_filter,
            query.title_filter.clone(),
            query.tag_filter.clone(),
            query.difficulty_filter.clone(),
            query.inner_id_order.clone(),
            query.problem_id_order.clone(),
            query.difficulty_order.clone(),
            query.limit,
            query.offset,
            pool,
            mongodb_database,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[get("/{region}/{inner_id}")]
pub async fn get_linked_problem(
    web::Path((region, inner_id)): web::Path<(String, i32)>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    check_solve_right(pool.clone(), logged_user.clone(), region.clone())?;

    let res = web::block(move || region::get_linked_problem(region, inner_id, pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct CreateRegionSubmissionBody {
    src: String,
    language: String,
}

#[post("/{region}/{inner_id}/submission")]
pub async fn create_submission(
    web::Path((region, inner_id)): web::Path<(String, i32)>,
    body: web::Json<CreateRegionSubmissionBody>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser,
    judge_actor: web::Data<JudgeActorAddr>,
) -> Result<HttpResponse, ServiceError> {
    check_solve_right(pool.clone(), logged_user.clone(), region.clone())?;

    let res = web::block(move || {
        region::create_submission(
            region,
            inner_id,
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
