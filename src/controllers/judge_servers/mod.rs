pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/judge_server")
            .service(web::resource("/heartbeat").route(web::post().to(handler::handle_heartbeat)))
            .service(web::resource("/info").route(web::post().to(handler::get_server_info)))
            .service(web::resource("/get_file").route(web::post().to(handler::get_file)))
    );
}