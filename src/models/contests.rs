use crate::schema::*;
use chrono::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "contests"]
pub struct RawContest {
    pub region: String,
    pub title: String,
    pub introduction: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub seal_time: Option<NaiveDateTime>,
    pub settings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestSettings {
    pub register_after_start: bool,
    pub view_before_start: bool,
    pub view_after_start: bool,
    pub submit_after_end: bool,
}