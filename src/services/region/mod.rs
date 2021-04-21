use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::region_links::*;
use crate::models::regions::*;
use actix_web::web;
use diesel::prelude::*;

pub fn get_list(
    self_type: Option<String>,
    limit: i32,
    offset: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<Vec<Region>> {
    let conn = &db_connection(&pool)?;

    use crate::schema::regions as regions_schema;
    let regions: Vec<Region> = regions_schema::table
        .filter(
            regions_schema::self_type
                .nullable()
                .eq(self_type.clone())
                .or(self_type.is_none()),
        )
        .limit(limit.into())
        .offset(offset.into())
        .load(conn)?;

    Ok(regions)
}

pub fn insert_problems(
    region: String,
    problem_ids: Vec<i32>,
    score: Option<i32>,
    pool: web::Data<Pool>,
) -> ServiceResult<Vec<CreateRegionLinksResult>> {
    let conn = &db_connection(&pool)?;

    use crate::schema::region_links as region_links_schema;

    let mut res = Vec::new();
    for problem_id in problem_ids {
        match diesel::insert_into(region_links_schema::table)
            .values(&RegionLink {
                region: region.clone(),
                problem_id: problem_id,
                score: Some(score.unwrap_or(100)),
            })
            .execute(conn) {
                Ok(_) => { 
                    res.push(CreateRegionLinksResult {
                        problem_id: problem_id,
                        is_success: true,
                    });
                },
                Err(_) => {
                    res.push(CreateRegionLinksResult {
                        problem_id: problem_id,
                        is_success: false,
                    });
                },
            }
    }

    Ok(res)
}