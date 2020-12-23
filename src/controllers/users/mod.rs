pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(handler::get)
            .service(handler::create)
    );
}