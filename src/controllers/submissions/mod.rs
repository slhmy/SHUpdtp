pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/submissions")
            .service(handler::create)
            .service(handler::get),
    );
}
