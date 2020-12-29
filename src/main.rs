#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;

mod cli_args;
mod errors;
mod database;
mod schema;
mod models;
mod services;
mod controllers;

use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer};
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};

#[actix_web::get("/")]
async fn hello() -> impl actix_web::Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init();

    // Get options
    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    let pool = database::pool::establish_connection(opt.clone());

    let domain = opt.domain.clone();
    let cookie_secret_key = opt.auth_secret_key.clone();
    let secure_cookie = opt.secure_cookie;
    let auth_duration = time::Duration::hours(i64::from(opt.auth_duration_in_hour));

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(Cors::default()
                .supports_credentials())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(&domain)
                    // Time from creation that cookie remains valid
                    .max_age_time(auth_duration)
                    // Restricted to https?
                    .secure(secure_cookie),
            ))
            .service(hello)
            .configure(controllers::users::route)
    })
    .bind(("0.0.0.0", opt.port))
    .unwrap()
    .run()
    .await
}