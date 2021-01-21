use actix_web::{web, HttpResponse, get, post, put, delete};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use crate::errors::ServiceError;
use crate::database::Pool;
use crate::models::users::LoggedUser;
use crate::services::problem;

#[post("")]
pub async fn batch_create(
    //logged_user: LoggedUser,
    mut payload: Multipart,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let mut bytes = web::BytesMut::new();
    // iterate over multipart stream
    let mut filename = None;
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        if filename.is_none() {
            filename = Some(content_type.get_filename().unwrap().to_owned());
        } else {
            // only accept one file
            if filename.clone().unwrap() != content_type.get_filename().unwrap() { continue; }
        }

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            bytes.extend_from_slice(&data);
        }
    }

    let res = web::block(move || problem::batch_create(&bytes, pool)).await.map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize)]
pub struct GetProblemListParams {
    id_filter: Option<i32>,
    title_filter: Option<String>,
    tag_filter: Option<Vec<String>>,
    difficulty_filter: Option<String>,
    id_order: Option<bool>,
    difficulty_order: Option<bool>,
    limit: i32,
    offset: i32,
}

#[get("")]
pub async fn get_list(
    query: web::Query<GetProblemListParams>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || problem::get_list(
        query.id_filter,
        query.title_filter.clone(),
        query.tag_filter.clone(),
        query.difficulty_filter.clone(),
        query.id_order.clone(),
        query.difficulty_order.clone(),
        query.limit,
        query.offset,
        pool
    )).await.map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[get("/{id}")]
pub async fn get(
    web::Path(id): web::Path<i32>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if logged_user.0.is_none() { return Err(ServiceError::Unauthorized); }
    let cur_user = logged_user.0.unwrap();
    if cur_user.role != "super" && cur_user.role != "admin" {
        let hint = "No permission.".to_string();
        return  Err(ServiceError::BadRequest(hint));
    }

    let res = web::block(move || problem::get(
        id,
        pool,
    )).await.map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[delete("/{id}")]
pub async fn delete(
    web::Path(id): web::Path<i32>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if logged_user.0.is_none() { return Err(ServiceError::Unauthorized); }
    let cur_user = logged_user.0.unwrap();
    if cur_user.role != "super" && cur_user.role != "admin" {
        let hint = "No permission.".to_string();
        return  Err(ServiceError::BadRequest(hint));
    }

    let res = web::block(move || problem::delete(
        id,
        pool,
    )).await.map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}