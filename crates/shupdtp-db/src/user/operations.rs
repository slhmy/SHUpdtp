use diesel::prelude::*;
use crate::schema::users as users_schema;
use server_core::{
    errors::ServiceResult,
    database::PooledConnection,
};
use super::models::*;

pub fn get_by_id(
    conn: &PooledConnection,
    id: i32,
) -> ServiceResult<User> {
    Ok(users_schema::table
        .filter(users_schema::id.eq(id))
        .first::<User>(conn)?
    )
}