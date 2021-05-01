pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/region")
            .service(handler::create)
            .service(handler::insert_problems)
            .service(handler::get_contest_list)
    );
}
