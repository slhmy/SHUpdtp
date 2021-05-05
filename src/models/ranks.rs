use crate::utils::*;
use chrono::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACMRank {
    pub region: String,
    pub last_updated_time: NaiveDateTime,
    pub columns: Vec<ACMRankColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACMRankColumn {
    pub rank: Option<i32>,
    pub user_id: i32,
    //temporary use account to show username
    pub account: String,
    pub total_accepted: i32,
    pub time_cost: i64,
    pub is_unrated: Option<bool>,
    pub problem_block: Vec<ACMProblemBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACMProblemBlock {
    pub inner_id: i32,
    pub is_accepted: Option<bool>,
    pub is_first_accepted: bool,
    pub is_sealed: bool,
    pub try_times: i32,
    pub last_submit_time: Option<NaiveDateTime>,
}
