use server_core::utils::encryption;
use crate::database::{db_connection, Pool};
use server_core::errors::{ServiceError, ServiceResult};
use crate::models::users::*;
use crate::models::utils::SizedList;
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

pub fn get_name(id: i32, pool: web::Data<Pool>) -> ServiceResult<String> {
    let conn = &db_connection(&pool)?;

    use crate::schema::users as users_schema;

    let name: String = users_schema::table
        .filter(users_schema::id.eq(id))
        .select(users_schema::account)
        .first(conn)?;

    Ok(name)
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

pub fn get_submissions_count(
    user_id: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<UserSubmissionCount> {
    let conn = &db_connection(&pool)?;

    use crate::schema::problems as problems_schema;
    use crate::schema::submissions as submissions_schema;

    let target = submissions_schema::table
        .filter(submissions_schema::user_id.eq(user_id))
        .inner_join(
            problems_schema::table.on(submissions_schema::problem_id.eq(problems_schema::id)),
        );

    let navie = target.filter(problems_schema::difficulty.lt(2.5));
    let easy = target.filter(
        problems_schema::difficulty
            .ge(2.5)
            .and(problems_schema::difficulty.lt(5.0)),
    );
    let middle = target.filter(
        problems_schema::difficulty
            .ge(5.0)
            .and(problems_schema::difficulty.lt(7.5)),
    );
    let hard = target.filter(problems_schema::difficulty.ge(7.5));

    let navie_submit_times: i64 = navie.count().get_result(conn)?;
    let navie_accept_times: i64 = navie
        .filter(submissions_schema::is_accepted.eq(true))
        .count()
        .get_result(conn)?;

    let easy_submit_times: i64 = easy.count().get_result(conn)?;
    let easy_accept_times: i64 = easy
        .filter(submissions_schema::is_accepted.eq(true))
        .count()
        .get_result(conn)?;

    let middle_submit_times: i64 = middle.count().get_result(conn)?;
    let middle_accept_times: i64 = middle
        .filter(submissions_schema::is_accepted.eq(true))
        .count()
        .get_result(conn)?;

    let hard_submit_times: i64 = hard.count().get_result(conn)?;
    let hard_accept_times: i64 = hard
        .filter(submissions_schema::is_accepted.eq(true))
        .count()
        .get_result(conn)?;

    let total_submit_times: i64 =
        navie_submit_times + easy_submit_times + middle_submit_times + hard_submit_times;
    let total_accept_times: i64 =
        navie_accept_times + easy_accept_times + middle_accept_times + hard_accept_times;

    Ok(UserSubmissionCount {
        total_submit_times: total_submit_times as i32,
        total_accept_times: total_accept_times as i32,
        navie_submit_times: navie_submit_times as i32,
        navie_accept_times: navie_accept_times as i32,
        easy_submit_times: easy_submit_times as i32,
        easy_accept_times: easy_accept_times as i32,
        middle_submit_times: middle_submit_times as i32,
        middle_accept_times: middle_accept_times as i32,
        hard_submit_times: hard_submit_times as i32,
        hard_accept_times: hard_accept_times as i32,
    })
}

pub fn get_submissions_time(
    user_id: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<Vec<UserSubmissionTime>> {
    let conn = &db_connection(&pool)?;

    use crate::schema::submissions as submissions_schema;

    let raw_times: Vec<chrono::NaiveDateTime> = submissions_schema::table
        .filter(submissions_schema::user_id.eq(user_id))
        .select(submissions_schema::submit_time)
        .order(submissions_schema::submit_time.desc())
        .load(conn)?;

    let mut time_count: Vec<UserSubmissionTime> = Vec::new();
    let mut last = 0;
    let mut first: bool = true;
    for time in raw_times {
        if first {
            time_count.push(UserSubmissionTime {
                date: time.date(),
                count: 1,
            });
            first = false;
        } else if time.date() == time_count[last].date {
            time_count[last].count += 1;
        } else {
            time_count.push(UserSubmissionTime {
                date: time.date(),
                count: 1,
            });
            last += 1;
        }
    }

    Ok(time_count)
}
