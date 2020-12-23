#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

mod cli_args;
mod errors;
mod database;
mod schema;
mod models;
mod services;
mod controllers;

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

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(hello)
            .configure(controllers::users::route)
    })
    .bind(("0.0.0.0", opt.port))
    .unwrap()
    .run()
    .await
}