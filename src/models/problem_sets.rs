use super::problems;
use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "problem_sets"]
pub struct ProblemSetInfo {
    pub name: String,
    pub introduction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSetColumn {
    pub problem: problems::OutProblem,
    pub submit_times: i32,
    pub accept_times: i32,
    pub error_times: i32,
}