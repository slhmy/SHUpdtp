use super::problems;
use crate::schema::*;

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

pub fn get_column_from_raw(
    raw: RawLinkedProblemColumn,
    mongodb_database: &mongodb::sync::Database,
) -> LinkedProblemColumn {
    use mongodb::bson::doc;
    if mongodb_database
        .collection("submission_statistics")
        .find_one(
            doc! {
                "problem_id": raw.problem_id,
                "region": raw.region.clone(),
            },
            None,
        )
        .unwrap()
        .is_none()
    {
        mongodb_database
            .collection("submission_statistics")
            .insert_one(
                doc! {
                    "problem_id": raw.problem_id,
                    "region": raw.region.clone(),
                    "submit_times": 0,
                    "accept_times": 0,
                    "error_times": 0,
                    "avg_max_time": 0,
                    "avg_max_memory": 0,
                },
                None,
            )
            .unwrap();
    }

    let mut submit_times = 0;
    let mut accept_times = 0;
    let mut error_times = 0;

    if let Some(doc) = mongodb_database
        .collection("submission_statistics")
        .find_one(
            doc! {
                "problem_id": raw.problem_id,
                "region": raw.region.clone(),
            },
            None,
        )
        .unwrap()
    {
        submit_times = doc.get("submit_times").unwrap().as_i32().unwrap();
        accept_times = doc.get("accept_times").unwrap().as_i32().unwrap();
        error_times = doc.get("error_times").unwrap().as_i32().unwrap();
    }

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
        submit_times: submit_times,
        accept_times: accept_times,
        error_times: error_times,
    }
}
