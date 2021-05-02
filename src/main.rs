#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod auth;
mod cli_args;
mod controllers;
mod database;
mod errors;
mod judge_actor;
mod models;
mod schema;
mod services;
mod statics;
mod utils;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer};

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
    let mongodb_client = mongodb::sync::Client::with_uri_str(&opt.mongodb_url).unwrap();
    let mongodb_database = mongodb_client.database("SHUpdtp");

    let domain = opt.domain.clone();
    let cookie_secret_key = opt.auth_secret_key.clone();
    let secure_cookie = opt.secure_cookie;
    let auth_duration = time::Duration::hours(i64::from(opt.auth_duration_in_hour));

    let judge_actor_addr = judge_actor::start_judge_actor(opt.clone());

    HttpServer::new(move || {
        App::new()
            .data(mongodb_database.clone())
            .data(pool.clone())
            .data(judge_actor::JudgeActorAddr {
                addr: judge_actor_addr.clone(),
            })
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret_key.as_bytes())
                    .name("auth")
                    .path("/")
                    // .domain(&domain)
                    // Time from creation that cookie remains valid
                    .max_age_time(auth_duration)
                    // .same_site(actix_web::cookie::SameSite::None)
                    // Restricted to https?
                    .secure(false),
            ))
            .service(hello)
            .configure(controllers::users::route)
            .configure(controllers::problems::route)
            .configure(controllers::judge_servers::route)
            .configure(controllers::submissions::route)
            .configure(controllers::samples::route)
            .configure(controllers::regions::route)
            .configure(controllers::problem_sets::route)
            .configure(controllers::contests::route)
    })
    .bind(("0.0.0.0", opt.port))
    .unwrap()
    .run()
    .await
}
