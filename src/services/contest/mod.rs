pub mod utils;

use crate::auth::encryption;
use crate::auth::region as region_access;
use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::access_control_list::*;
use crate::models::contests::*;
use crate::models::region_access_settings::*;
use crate::models::regions::*;
use crate::models::utils::SizedList;
use actix_web::web;
use chrono::*;
use diesel::prelude::*;

pub fn create(
    region: String,
    title: String,
    introduction: Option<String>,
    start_time: Option<NaiveDateTime>,
    end_time: Option<NaiveDateTime>,
    seal_time: Option<NaiveDateTime>,
    settings: ContestSettings,
    password: Option<String>,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    utils::check_settings_legal(settings.clone())?;

    let conn = &db_connection(&pool)?;

    use crate::schema::regions as regions_schema;
    diesel::insert_into(regions_schema::table)
        .values(&Region {
            name: region.clone(),
            self_type: "contest".to_owned(),
            title: title.clone(),
            has_access_setting: true,
            introduction: introduction.clone(),
        })
        .execute(conn)?;

    use crate::schema::contests as contests_schema;
    diesel::insert_into(contests_schema::table)
        .values(&RawContest {
            region: region.clone(),
            title: title,
            introduction: introduction,
            start_time: start_time,
            end_time: end_time,
            seal_time: seal_time,
            settings: serde_json::to_string(&settings).unwrap(),
        })
        .execute(conn)?;

    let (salt, hash) = {
        if let Some(inner_data) = password {
            let salt = encryption::make_salt();
            let hash = encryption::make_hash(&inner_data, &salt).to_vec();
            (Some(salt), Some(hash))
        } else {
            (None, None)
        }
    };

    use crate::schema::region_access_settings as region_access_settings_schema;
    diesel::insert_into(region_access_settings_schema::table)
        .values(&RegionAccessSetting {
            region: region,
            salt: salt,
            hash: hash,
        })
        .execute(conn)?;

    Ok(())
}

pub fn get_contest_list(
    title_filter: Option<String>,
    limit: i32,
    offset: i32,
    user_id: Option<i32>,
    pool: web::Data<Pool>,
) -> ServiceResult<SizedList<SlimContest>> {
    let conn = &db_connection(&pool)?;

    let title_filter = if let Some(inner_data) = title_filter {
        Some(String::from("%") + &inner_data.as_str().replace(" ", "%") + "%")
    } else {
        None
    };

    use crate::schema::contests as contests_schema;
    let target = contests_schema::table.filter(
        contests_schema::title
            .nullable()
            .like(title_filter.clone())
            .or(title_filter.is_none()),
    );

    let total: i64 = target.clone().count().get_result(conn)?;

    let raw_contests = target
        .offset(offset.into())
        .limit(limit.into())
        .load::<RawContest>(conn)?;

    let mut res = Vec::new();
    for raw_contest in raw_contests {
        let mut t = SlimContest::from(raw_contest);

        let access_setting = region_access::read_access_setting(conn, t.region.clone())?;
        if access_setting.hash.is_some() {
            t.need_pass = true;
        }

        if let Some(inner_data) = user_id {
            if region_access::check_acl(conn, inner_data, t.region.clone()).is_ok() {
                t.is_registered = true;
            }
        }

        res.push(t);
    }

    Ok(SizedList {
        total: total,
        list: res,
    })
}

pub fn register(
    region: String,
    maybe_password: Option<String>,
    user_id: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    use crate::schema::region_access_settings as region_access_settings_schema;
    let region_access_setting: RegionAccessSetting = region_access_settings_schema::table
        .filter(region_access_settings_schema::region.eq(region.clone()))
        .first(conn)?;

    if let Some(password) = maybe_password {
        let hash =
            encryption::make_hash(&password, &region_access_setting.clone().salt.unwrap()).to_vec();
        if Some(hash) != region_access_setting.hash {
            let hint = "Password is wrong.".to_string();
            return Err(ServiceError::BadRequest(hint));
        }
    } else {
        let hint = "Password not given.".to_string();
        return Err(ServiceError::BadRequest(hint));
    }

    use crate::schema::access_control_list as access_control_list_schema;
    diesel::insert_into(access_control_list_schema::table)
        .values(&AcessControlList { region, user_id })
        .execute(conn)?;

    Ok(())
}
