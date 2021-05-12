use super::problems;
use crate::schema::*;
use crate::errors::ServiceResult;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "region_links"]
pub struct RegionLink {
    pub region: String,
    pub inner_id: i32,
    pub problem_id: i32,
    pub score: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRegionLinksResult {
    pub problem_id: i32,
    pub inner_id: Option<i32>,
    pub is_success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct RawLinkedProblemColumn {
    pub region: String,
    pub inner_id: i32,
    pub problem_id: i32,
    pub problem_title: String,
    pub problem_tags: Vec<String>,
    pub problem_difficulty: f64,
    pub is_released: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedProblemColumn {
    pub region: String,
    pub inner_id: i32,
    pub out_problem: problems::OutProblem,
    pub submit_times: i32,
    pub accept_times: i32,
    pub error_times: i32,
}

use crate::database::*;
use crate::models::statistics::get_results;
pub fn get_column_from_raw(
    conn: &PooledConnection,
    raw: RawLinkedProblemColumn,
) -> ServiceResult<LinkedProblemColumn> {
    let statistic = get_results(conn, raw.region.clone(), raw.problem_id)?;

    Ok(
        LinkedProblemColumn {
            region: raw.region,
            inner_id: raw.inner_id,
            out_problem: problems::OutProblem {
                id: raw.problem_id,
                info: problems::ProblemInfo {
                    title: raw.problem_title,
                    tags: raw.problem_tags,
                    difficulty: raw.problem_difficulty,
                },
                is_released: raw.is_released,
            },
            submit_times: statistic.submit_times,
            accept_times: statistic.accept_times,
            error_times: statistic.error_times,
        }
    )
}


