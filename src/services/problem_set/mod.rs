use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::problem_sets::*;
use actix_web::web;
use diesel::prelude::*;

pub fn create(
    name: String,
    introduction: Option<String>,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    use crate::schema::problem_sets as problem_sets_schema;
    diesel::insert_into(problem_sets_schema::table)
        .values(&ProblemSetInfo {
            name: name,
            introduction: introduction,
        })
        .execute(conn)?;

    Ok(())
}
