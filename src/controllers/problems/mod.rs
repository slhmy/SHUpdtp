pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/problems")
            .service(handler::batch_create)
            .service(handler::get_list)
            .service(handler::get)
            .service(handler::delete)
    );
}