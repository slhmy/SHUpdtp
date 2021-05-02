use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::users::*;
use crate::models::utils::SizedList;
use crate::auth::encryption;
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
        let salt = encryption::make_salt();
        let hash = encryption::make_hash(&password, &salt).to_vec();
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

    let (new_salt, new_hash) = if let Some(inner_data) = new_password {
        let salt = encryption::make_salt();
        let hash = encryption::make_hash(&inner_data, &salt).to_vec();
        (Some(salt), Some(hash))
    } else {
        (None, None)
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
) -> ServiceResult<SizedList<OutUser>> {
    let account_filter = if let Some(inner_data) = account_filter {
        Some(String::from("%") + &inner_data.as_str().replace(" ", "%") + "%")
    } else {
        None
    };

    let mobile_filter = if let Some(inner_data) = mobile_filter {
        Some(String::from("%") + &inner_data.as_str().replace(" ", "%") + "%")
    } else {
        None
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
        );

    let total: i64 = target.clone().count().get_result(conn)?;

    let target = target.offset(offset.into()).limit(limit.into());

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

    Ok(SizedList {
        total: total,
        list: out_users,
    })
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
        let hash = encryption::make_hash(&password, &user.clone().salt.unwrap()).to_vec();
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
            "sup" => Ok(config.sup.clone().unwrap_or_default()),
            "admin" => Ok(config.admin.clone().unwrap_or_default()),
            "student" => Ok(config.student.clone().unwrap_or_default()),
            "teacher" => Ok(config.teacher.clone().unwrap_or_default()),
            _ => Ok(config.others.clone().unwrap_or_default()),
        },
        None => {
            let hint = "Path not set in config.".to_string();
            Err(ServiceError::BadRequest(hint))
        }
    }
}

pub fn delete(id: i32, pool: web::Data<Pool>) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    use crate::schema::users as users_schema;
    diesel::delete(users_schema::table.filter(users_schema::id.eq(id))).execute(conn)?;

    Ok(())
}
