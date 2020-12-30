use actix_web::{web, HttpResponse, FromRequest, Error, HttpRequest, get, post, put};
use actix_web::dev::Payload;
use actix_identity::{Identity, RequestIdentity};
use crate::errors::ServiceError;
use crate::database::Pool;
use crate::services::user;
use crate::models::users::{LoggedUser, SlimUser};

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity = req.get_identity();

        let slim_user = if let Some(identity) = identity {
            match serde_json::from_str::<SlimUser>(&identity) {
                Err(e) => return futures::future::err(e.into()),
                Ok(y) => Ok(Some(y)),
            }
        } else {
            Ok(None)
        };

        futures::future::ready(slim_user.map(LoggedUser))
    }
}

#[derive(Deserialize)]
pub struct GetUserListBody {
    id_filter: Option<i32>,
    account_filter: Option<String>,
    mobile_filter: Option<String>,
    role_filter: Option<String>,
    id_order: Option<bool>,
    limit: i32,
    offset: i32,
}

#[get("")]
pub async fn get_list(
    body: web::Json<GetUserListBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || user::get_list(
        body.id_filter,
        body.account_filter.clone(),
        body.mobile_filter.clone(),
        body.role_filter.clone(),
        body.id_order.clone(),
        body.limit,
        body.offset,
        pool
    )).await.map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct CreateUserBody {
    account: String,
    password: Option<String>,
    mobile: Option<String>,
    role: String,
}

#[post("")]
pub async fn create(
    body: web::Json<CreateUserBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || user::create(
        body.account.clone(),
        body.password.clone(),
        body.mobile.clone(),
        body.role.clone(),
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
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || user::get(
        id,
        pool
    )).await.map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct UpdateUserBody {
    new_account: Option<String>,
    new_password: Option<String>,
    new_mobile: Option<String>,
    new_role: Option<String>,
}

#[put("/{id}")]
pub async fn update(
    web::Path(id): web::Path<i32>,
    body: web::Json<UpdateUserBody>,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if logged_user.0.is_none() { return Err(ServiceError::Unauthorized); }
    let cur_user = logged_user.0.unwrap();
    if cur_user.id != id && cur_user.role != "super" && cur_user.role != "admin" {
        let hint = "No permission.".to_string();
        return  Err(ServiceError::BadRequest(hint));
    }

    let res = web::block(move || user::update(
        id,
        body.new_account.clone(),
        body.new_password.clone(),
        body.new_mobile.clone(),
        body.new_role.clone(),
        pool,
    )).await.map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[derive(Deserialize)]
pub struct LoginBody {
    account: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    body: web::Json<LoginBody>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || user::login(
        body.account.clone(), 
        body.password.clone(), 
        pool
    )).await.map_err(|e| {
        eprintln!("{}", e);
        e
    })?;
    
    let user_string = serde_json::to_string(&res)
        .map_err(|_| ServiceError::InternalServerError)?;
    info!("user_string={}", user_string);
    identity.remember(user_string);
    Ok(HttpResponse::Ok().json(res))
}

#[post("/logout")]
pub fn logout(identity: Identity) -> HttpResponse {
    identity.forget();
    HttpResponse::Ok().finish()
}