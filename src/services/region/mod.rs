use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::region_links::*;
use crate::models::regions::*;
use crate::models::utils::SizedList;
use actix_web::web;
use diesel::prelude::*;

pub fn get_list(
    self_type: Option<String>,
    limit: i32,
    offset: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<SizedList<Region>> {
    let conn = &db_connection(&pool)?;

    use crate::schema::regions as regions_schema;
    let target = regions_schema::table
        .filter(
            regions_schema::self_type
                .nullable()
                .eq(self_type.clone())
                .or(self_type.is_none()),
        );

    let total: i64 = target.clone().count().get_result(conn)?;

    let regions: Vec<Region> = target
        .limit(limit.into())
        .offset(offset.into())
        .load(conn)?;

    Ok(SizedList{
        total: total,
        list: regions,
    })
}

pub fn insert_problems(
    region: String,
    problem_ids: Vec<i32>,
    score: Option<i32>,
    pool: web::Data<Pool>,
) -> ServiceResult<Vec<CreateRegionLinksResult>> {
    let conn = &db_connection(&pool)?;

    use crate::schema::problems as problems_schema;
    use crate::schema::region_links as region_links_schema;

    let mut target_id: i32 = region_links_schema::table
        .select(region_links_schema::inner_id)
        .filter(region_links_schema::region.eq(region.clone()))
        .order(region_links_schema::inner_id.desc())
        .first(conn)
        .unwrap_or(0);
    target_id += 1;

    let mut res = Vec::new();
    for problem_id in problem_ids {
        match diesel::insert_into(region_links_schema::table)
            .values(&RegionLink {
                region: region.clone(),
                inner_id: target_id,
                problem_id: problem_id,
                score: Some(score.unwrap_or(100)),
            })
            .execute(conn)
        {
            Ok(_) => {
                diesel::update(problems_schema::table.filter(problems_schema::id.eq(problem_id)))
                    .set(problems_schema::is_released.eq(true))
                    .execute(conn)
                    .expect("Error changing problem's release state.");
                res.push(CreateRegionLinksResult {
                    problem_id: problem_id,
                    inner_id: Some(target_id),
                    is_success: true,
                });
                target_id += 1;
            }
            Err(_) => {
                res.push(CreateRegionLinksResult {
                    problem_id: problem_id,
                    inner_id: None,
                    is_success: false,
                });
            }
        }
    }

    Ok(res)
}
