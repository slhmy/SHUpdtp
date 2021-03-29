pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/judge_server")
            .service(handler::handle_heartbeat)
            .service(handler::get_server_info),
    );
}
