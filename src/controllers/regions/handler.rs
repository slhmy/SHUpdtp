use crate::database::Pool;
use crate::errors::ServiceError;
// use crate::models::users::LoggedUser;
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
        region::get_list(
            query.self_type.clone(),
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
