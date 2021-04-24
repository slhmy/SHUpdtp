use crate::database::{Pool, SyncMongo};
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
pub struct GetProblemSetListParams {
    name_filter: Option<String>,
    limit: i32,
    offset: i32,
}

#[get("")]
pub async fn get_set_list(
    query: web::Query<GetProblemSetListParams>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        problem_set::get_set_list(query.name_filter.clone(), query.limit, query.offset, pool)
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
pub struct GetProblemSetColumnListParams {
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
pub async fn get_item_list(
    web::Path(region): web::Path<String>,
    query: web::Query<GetProblemSetColumnListParams>,
    pool: web::Data<Pool>,
    mongodb_database: web::Data<SyncMongo>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        problem_set::get_item_list(
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
