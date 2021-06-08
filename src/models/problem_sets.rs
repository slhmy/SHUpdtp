use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "problem_sets"]
pub struct ProblemSetInfo {
    pub region: String,
    pub title: String,
    pub introduction: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "problem_sets"]
pub struct ProblemSetForm {
    pub title: Option<String>,
    pub introduction: Option<String>,
}
