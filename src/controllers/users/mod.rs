pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(handler::get)
            .service(handler::create)
            .service(handler::update)
            .service(handler::get_list)
            .service(handler::login)
            .service(handler::logout)
    );
}