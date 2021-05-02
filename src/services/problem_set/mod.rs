use crate::database::{db_connection, Pool, SyncMongo};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::problem_sets;
use crate::models::problem_sets::*;
use crate::models::regions::*;
use crate::models::utils::SizedList;
use actix_web::web;
use diesel::prelude::*;

pub fn create(
    region: String,
    title: String,
    introduction: Option<String>,
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

    use crate::schema::problem_sets as problem_sets_schema;
    diesel::insert_into(problem_sets_schema::table)
        .values(&ProblemSetInfo {
            region: region,
            title: title,
            introduction: introduction,
        })
        .execute(conn)?;

    Ok(())
}

pub fn get_set_list(
    title_filter: Option<String>,
    limit: i32,
    offset: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<SizedList<ProblemSetInfo>> {
    let conn = &db_connection(&pool)?;

    let title_filter = if let Some(inner_data) = title_filter {
        Some(String::from("%") + &inner_data.as_str().replace(" ", "%") + "%")
    } else {
        None
    };

    use crate::schema::problem_sets as problem_sets_schema;
    let target = problem_sets_schema::table.filter(
        problem_sets_schema::title
            .nullable()
            .like(title_filter.clone())
            .or(title_filter.is_none()),
    );

    let total: i64 = target.clone().count().get_result(conn)?;

    let res = target
        .offset(offset.into())
        .limit(limit.into())
        .load(conn)?;

    Ok(SizedList {
        total: total,
        list: res,
    })
}

pub fn get_item_list(
    region: String,
    inner_id_filter: Option<i32>,
    problem_id_filter: Option<i32>,
    title_filter: Option<String>,
    tag_filter: Option<Vec<String>>,
    difficulty_filter: Option<String>,
    id_order: Option<bool>,
    problem_id_order: Option<bool>,
    difficulty_order: Option<bool>,
    limit: i32,
    offset: i32,
    pool: web::Data<Pool>,
    mongodb_database: web::Data<SyncMongo>,
) -> ServiceResult<SizedList<ProblemSetColumn>> {
    let conn = &db_connection(&pool)?;

    use crate::schema::regions as regions_schema;
    let count: i64 = regions_schema::table
        .filter(regions_schema::name.eq(region.clone()))
        .filter(regions_schema::self_type.eq("problem_set".to_owned()))
        .count()
        .get_result(conn)?;
    if count != 1 {
        let hint = "Bad region.".to_string();
        return Err(ServiceError::BadRequest(hint));
    }

    let title_filter = if let Some(inner_data) = title_filter {
        Some(String::from("%") + &inner_data.as_str().replace(" ", "%") + "%")
    } else {
        None
    };

    let tag_filter: Vec<String> = if let Some(inner_data) = tag_filter {
        inner_data.clone()
    } else {
        Vec::<String>::new()
    };

    let (min_difficulty, max_difficulty) = if difficulty_filter.is_none() {
        (0.0, 10.0)
    } else {
        match difficulty_filter.unwrap().as_str() {
            "Navie" => (0.0, 2.5),
            "Easy" => (2.5, 5.0),
            "Middle" => (5.0, 7.5),
            "Hard" => (7.5, 10.0),
            _ => (0.0, 10.0),
        }
    };

    use crate::schema::problems as problems_schema;
    use crate::schema::region_links as region_links_schema;
    let target = region_links_schema::table
        .inner_join(
            problems_schema::table.on(problems_schema::id.eq(region_links_schema::problem_id)),
        )
        .filter(region_links_schema::region.eq(region))
        .filter(
            region_links_schema::inner_id
                .nullable()
                .eq(inner_id_filter)
                .or(inner_id_filter.is_none()),
        )
        .filter(
            problems_schema::id
                .nullable()
                .eq(problem_id_filter)
                .or(problem_id_filter.is_none()),
        )
        .filter(
            problems_schema::tags
                .overlaps_with(tag_filter.clone())
                .or(tag_filter.is_empty()),
        )
        .filter(
            problems_schema::title
                .nullable()
                .like(title_filter.clone())
                .or(title_filter.is_none()),
        )
        .filter(problems_schema::difficulty.between(min_difficulty, max_difficulty));

    let total: i64 = target.clone().count().get_result(conn)?;

    let target = target.offset(offset.into()).limit(limit.into()).select((
        region_links_schema::region,
        region_links_schema::inner_id,
        problems_schema::id,
        problems_schema::title,
        problems_schema::tags,
        problems_schema::difficulty,
        problems_schema::is_released,
    ));

    let columns: Vec<RawProblemSetColumn> = match id_order {
        None => match problem_id_order {
            None => match difficulty_order {
                None => target.load(conn)?,
                Some(true) => target.order(problems_schema::difficulty.asc()).load(conn)?,
                Some(false) => target
                    .order(problems_schema::difficulty.desc())
                    .load(conn)?,
            },
            Some(true) => target.order(problems_schema::id.asc()).load(conn)?,
            Some(false) => target.order(problems_schema::id.desc()).load(conn)?,
        },
        Some(true) => target
            .order(region_links_schema::inner_id.asc())
            .load(conn)?,
        Some(false) => target
            .order(region_links_schema::inner_id.desc())
            .load(conn)?,
    };

    let out_columns = {
        let mut res = Vec::new();
        for column in columns {
            res.push(problem_sets::get_column_from_raw(column, &mongodb_database));
        }
        res
    };

    Ok(SizedList {
        total: total,
        list: out_columns,
    })
}
