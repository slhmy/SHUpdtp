mod utils;

use crate::errors::{ ServiceResult, ServiceError };
use crate::database::{db_connection, Pool};
use crate::models::users::{ User, InsertableUser, OutUser };
use actix_web::web;
use diesel::prelude::*;
use utils::{ make_salt, make_hash };

pub fn create(
    name: String,
    password: Option<String>,
    mobile: Option<String>,
    role: String,
    pool: web::Data<Pool>
) -> ServiceResult<()>
{
    if password.is_none() && mobile.is_none() {
        let hint = "Please provide password | mobile".to_string();
        return Err(ServiceError::BadRequest(hint));
    }

    let (salt, hash) = if password.is_none() { (None, None) } else {
        let salt = make_salt();
        let hash = make_hash(&password.unwrap(), &salt).to_vec();
        (Some(salt), Some(hash))
    };

    let conn = &db_connection(&pool)?;

    use crate::schema::users::dsl::users;
    diesel::insert_into(users)
        .values(&InsertableUser{
            salt: salt,
            hash: hash,
            name: name,
            mobile: mobile,
            role: role,
        }).execute(conn)?;

    Ok(())
}

pub fn get(
    id: i32,
    pool: web::Data<Pool>
) -> ServiceResult<OutUser>
{
    let conn = &db_connection(&pool)?;

    use crate::schema::users;
    let user: User = users::table.filter(users::id.eq(id)).first(conn)?;

    Ok(OutUser::from(user))
}