pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/samples")
            .service(handler::create)
            .service(handler::get_list)
            .service(handler::get)
            .service(handler::delete)
    );
}