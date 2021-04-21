pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/problem_sets")
            .service(handler::create)
            .service(handler::insert_problems)
    );
}
