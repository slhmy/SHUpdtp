mod utils;

use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::users::*;
use actix_web::web;
use diesel::prelude::*;

pub fn create(
    account: String,
    password: String,
    mobile: Option<String>,
    role: String,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let (salt, hash) = {
        let salt = utils::make_salt();
        let hash = utils::make_hash(&password, &salt).to_vec();
        (Some(salt), Some(hash))
    };

    let conn = &db_connection(&pool)?;

    use crate::schema::users as users_schema;
    diesel::insert_into(users_schema::table)
        .values(&InsertableUser {
            salt: salt,
            hash: hash,
            account: account,
            mobile: mobile,
            role: role,
        })
        .execute(conn)?;

    Ok(())
}

pub fn get(id: i32, pool: web::Data<Pool>) -> ServiceResult<OutUser> {
    let conn = &db_connection(&pool)?;

    use crate::schema::users as users_schema;
    let user: User = users_schema::table
        .filter(users_schema::id.eq(id))
        .first(conn)?;

    Ok(OutUser::from(user))
}

pub fn update(
    id: i32,
    new_account: Option<String>,
    new_password: Option<String>,
    new_mobile: Option<String>,
    new_role: Option<String>,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    let (new_salt, new_hash) = if new_password.is_none() {
        (None, None)
    } else {
        let salt = utils::make_salt();
        let hash = utils::make_hash(&new_password.unwrap(), &salt).to_vec();
        (Some(salt), Some(hash))
    };

    use crate::schema::users as users_schema;
    diesel::update(users_schema::table.filter(users_schema::id.eq(id)))
        .set(UserForm {
            salt: new_salt,
            hash: new_hash,
            account: new_account,
            mobile: new_mobile,
            role: new_role,
        })
        .execute(conn)?;

    Ok(())
}

pub fn get_list(
    id_filter: Option<i32>,
    account_filter: Option<String>,
    mobile_filter: Option<String>,
    role_filter: Option<String>,
    id_order: Option<bool>,
    limit: i32,
    offset: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<Vec<OutUser>> {
    let account_filter = if account_filter.is_none() {
        None
    } else {
        Some(String::from("%") + &account_filter.unwrap().as_str().replace(" ", "%") + "%")
    };

    let mobile_filter = if mobile_filter.is_none() {
        None
    } else {
        Some(String::from("%") + &mobile_filter.unwrap().as_str().replace(" ", "%") + "%")
    };

    let conn = &db_connection(&pool)?;

    use crate::schema::users as users_schema;
    let target = users_schema::table
        .filter(
            users_schema::id
                .nullable()
                .eq(id_filter)
                .or(id_filter.is_none()),
        )
        .filter(
            users_schema::account
                .nullable()
                .like(account_filter.clone())
                .or(account_filter.is_none()),
        )
        .filter(
            users_schema::mobile
                .like(mobile_filter.clone())
                .or(mobile_filter.is_none()),
        )
        .filter(
            users_schema::role
                .nullable()
                .eq(role_filter.clone())
                .or(role_filter.is_none()),
        )
        .limit(limit.into())
        .offset(offset.into());

    let users: Vec<User> = match id_order {
        None => target.load(conn)?,
        Some(true) => target.order(users_schema::id.asc()).load(conn)?,
        Some(false) => target.order(users_schema::id.desc()).load(conn)?,
    };

    let out_users = {
        let mut res = Vec::new();
        for user in users {
            res.push(OutUser::from(user));
        }
        res
    };

    Ok(out_users)
}

pub fn login(account: String, password: String, pool: web::Data<Pool>) -> ServiceResult<SlimUser> {
    let conn = &db_connection(&pool)?;

    use crate::schema::users as users_schema;
    let user: User = users_schema::table
        .filter(users_schema::account.eq(account))
        .first(conn)?;

    if user.hash.is_none() || user.salt.is_none() {
        let hint = "Password was not set.".to_string();
        Err(ServiceError::BadRequest(hint))
    } else {
        let hash = utils::make_hash(&password, &user.clone().salt.unwrap()).to_vec();
        if Some(hash) == user.hash {
            Ok(SlimUser::from(user))
        } else {
            let hint = "Password is wrong.".to_string();
            Err(ServiceError::BadRequest(hint))
        }
    }
}

pub fn get_permitted_methods(role: String, path: String) -> ServiceResult<Vec<String>> {
    use crate::statics::AUTH_CONFIG;
    match AUTH_CONFIG.get(&path) {
        Some(config) => match role.as_str() {
            "sup" => Ok(config.sup.clone().unwrap_or(vec![])),
            "admin" => Ok(config.admin.clone().unwrap_or(vec![])),
            "student" => Ok(config.student.clone().unwrap_or(vec![])),
            "teacher" => Ok(config.teacher.clone().unwrap_or(vec![])),
            _ => Ok(config.others.clone().unwrap_or(vec![])),
        },
        None => {
            let hint = "Path not set in config.".to_string();
            Err(ServiceError::BadRequest(hint))
        }
    }
}
