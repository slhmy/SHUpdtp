pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contests")
            .service(handler::create)
            .service(handler::get_contest_list)
            .service(handler::register)
            .service(handler::get_acm_rank)
            .service(handler::delete)
            .service(handler::update),
    );
}
