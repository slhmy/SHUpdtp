use crate::database::{db_connection, Pool, SyncMongo};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::contests::*;
use crate::models::region_access_settings::*;
use crate::models::regions::*;
use crate::models::utils::SizedList;
use crate::services::user::utils;
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
    let conn = &db_connection(&pool)?;

    use crate::schema::regions as regions_schema;
    diesel::insert_into(regions_schema::table)
        .values(&Region {
            name: region.clone(),
            self_type: "problem_set".to_owned(),
            title: title.clone(),
            has_access_policy: false,
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

    if let Some(inner_data) = password {
        let (salt, hash) = {
            let salt = utils::make_salt();
            let hash = utils::make_hash(&inner_data, &salt).to_vec();
            (Some(salt), Some(hash))
        };

        use crate::schema::region_access_settings as region_access_settings_schema;
        diesel::insert_into(region_access_settings_schema::table)
            .values(&RegionAccessSettings {
                region: region,
                salt: salt,
                hash: hash,
            })
            .execute(conn)?;
    }

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

    let res = Vec::new();
    for raw_contest in raw_contests {
        let mut t = SlimContest::from(raw_contest);

        use crate::schema::region_access_settings as region_access_settings_schema;
        if region_access_settings_schema::table
            .filter(region_access_settings_schema::region.eq(t.region.clone()))
            .count()
            .get_result::<i64>(conn)?
            > 0
        {
            t.need_pass = true;
        }

        if let Some(inner_data) = user_id {
            use crate::schema::access_control_list as access_control_list_schema;
            if access_control_list_schema::table
                .filter(access_control_list_schema::region.eq(t.region.clone()))
                .filter(access_control_list_schema::user_id.eq(inner_data))
                .count()
                .get_result::<i64>(conn)?
                > 0
            {
                t.is_registered = true;
            }
        }
    }

    Ok(SizedList {
        total: total,
        list: res,
    })
}
