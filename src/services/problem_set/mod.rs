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
    name: String,
    introduction: Option<String>,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    use crate::schema::regions as regions_schema;
    diesel::insert_into(regions_schema::table)
        .values(&Region {
            name: region.clone(),
            self_type: "problem_set".to_owned(),
        })
        .execute(conn)?;

    use crate::schema::problem_sets as problem_sets_schema;
    diesel::insert_into(problem_sets_schema::table)
        .values(&ProblemSetInfo {
            region: region,
            name: name,
            introduction: introduction,
        })
        .execute(conn)?;

    Ok(())
}

pub fn get_list(
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
        .filter(regions_schema::name.eq(region))
        .filter(regions_schema::self_type.eq("problem_set".to_owned()))
        .count()
        .get_result(conn)?;
    if count != 1 {
        let hint = "Bad region.".to_string();
        return Err(ServiceError::BadRequest(hint));
    }

    let title_filter = if title_filter.is_none() {
        None
    } else {
        Some(String::from("%") + &title_filter.unwrap().as_str().replace(" ", "%") + "%")
    };

    let tag_filter: Vec<String> = if tag_filter.is_some() {
        if tag_filter.clone().unwrap().len() > 0 {
            tag_filter.unwrap()
        } else {
            Vec::<String>::new()
        }
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
        .filter(problems_schema::difficulty.between(min_difficulty, max_difficulty))
        .limit(limit.into());

    let total: i64 = target.clone().count().get_result(conn)?;

    let target = target
        .offset(offset.into())
        .select((
            region_links_schema::region,
            region_links_schema::inner_id,
            problems_schema::id,
            problems_schema::title,
            problems_schema::tags,
            problems_schema::difficulty,
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
