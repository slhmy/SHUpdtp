use crate::errors::ServiceResult;
use diesel::prelude::*;

pub fn get_self_type(region: String, conn: &PgConnection) -> ServiceResult<String> {
    use crate::schema::regions as regions_schema;

    Ok(regions_schema::table
        .filter(regions_schema::name.eq(region))
        .select(regions_schema::self_type)
        .first(conn)?)
}
