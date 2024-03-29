use diesel::prelude::*;
use server_core::errors::ServiceResult;

pub fn get_self_type(region: String, db_connection: &PgConnection) -> ServiceResult<String> {
    use crate::schema::regions as regions_schema;

    Ok(regions_schema::table
        .filter(regions_schema::name.eq(region))
        .select(regions_schema::self_type)
        .first(db_connection)?)
}
