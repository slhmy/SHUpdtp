use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use actix_web::web;
use diesel::prelude::*;

pub fn get_self_type(region: String, pool: web::Data<Pool>) -> ServiceResult<String> {
    let conn = &db_connection(&pool)?;

    use crate::schema::regions as regions_schema;

    Ok(regions_schema::table
        .filter(regions_schema::name.eq(region))
        .select(regions_schema::self_type)
        .first(conn)?)
}
