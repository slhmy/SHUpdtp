pub mod handler;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/regions")
            .service(handler::get_list)
            .service(handler::insert_problems)
            .service(handler::get_linked_problem_column_list)
            .service(handler::get_linked_problem)
            .service(handler::create_submission)
            .service(handler::delete_problem),
    );
}
