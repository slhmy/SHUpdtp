pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(handler::me)
            .service(handler::get_permitted_methods)
            .service(handler::get_name)
            .service(handler::get)
            .service(handler::create)
            .service(handler::update)
            .service(handler::get_list)
            .service(handler::login)
            .service(handler::logout)
            .service(handler::delete),
    );
}
