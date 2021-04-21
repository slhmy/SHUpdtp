use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::problem_sets::*;
use crate::models::regions::*;
use actix_web::web;
use diesel::prelude::*;

pub fn create(
    region: String,
    name: String,
    introduction: Option<String>,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    use crate::schema::regions as regions_schema;
    diesel::insert_into(regions_schema::table)
        .values(&Region {
            name: region.clone(),
            self_type: "problem_set".to_owned(),
        })
        .execute(conn)?;

    use crate::schema::problem_sets as problem_sets_schema;
    diesel::insert_into(problem_sets_schema::table)
        .values(&ProblemSetInfo {
            region: region,
            name: name,
            introduction: introduction,
        })
        .execute(conn)?;

    Ok(())
}
