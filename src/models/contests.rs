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
pub struct SlimContest {
    pub region: String,
    pub title: String,
    pub introduction: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub seal_time: Option<NaiveDateTime>,
    pub is_registered: bool,
    pub need_pass: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestSettings {
    pub register_after_start: bool,
    pub view_before_start: bool,
    pub view_after_end: bool,
    pub public_after_end: bool,
    pub submit_after_end: bool,
}

impl From<RawContest> for SlimContest {
    fn from(raw: RawContest) -> Self {
        Self {
            region: raw.region,
            title: raw.title,
            introduction: raw.introduction,
            start_time: raw.start_time,
            end_time: raw.end_time,
            seal_time: raw.seal_time,
            is_registered: false,
            need_pass: false,
        }
    }
}

impl Default for ContestSettings {
    fn default() -> Self {
        Self {
            register_after_start: true,
            view_before_start: false,
            view_after_end: true,
            public_after_end: false,
            submit_after_end: true,
        }
    }
}
