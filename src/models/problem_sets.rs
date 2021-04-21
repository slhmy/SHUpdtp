use super::problems;
use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "problem_sets"]
pub struct ProblemSetInfo {
    pub region: String,
    pub name: String,
    pub introduction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct RawProblemSetColumn {
    pub set_name: String,
    pub problem_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSetColumn {
    pub set_name: String,
    pub problem: problems::OutProblem,
    pub submit_times: i32,
    pub accept_times: i32,
    pub error_times: i32,
}
