use crate::schema::*;
use crate::utils::get_cur_naive_date_time;
use chrono::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "contests"]
pub struct RawContest {
    pub region: String,
    pub title: String,
    pub introduction: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub seal_time: Option<NaiveDateTime>,
    pub settings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contest {
    pub region: String,
    pub title: String,
    pub introduction: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub seal_time: Option<NaiveDateTime>,
    pub settings: ContestSettings,
    pub state: String,
}

impl From<RawContest> for Contest {
    fn from(raw: RawContest) -> Self {
        let mut res = Self {
            region: raw.region,
            title: raw.title,
            introduction: raw.introduction,
            start_time: raw.start_time,
            end_time: raw.end_time,
            seal_time: raw.seal_time,
            settings: serde_json::from_str(&raw.settings).unwrap(),
            state: format!("{}", ContestState::Ended),
        };
        res.state = format!(
            "{}",
            get_contest_state(res.clone(), get_cur_naive_date_time())
        );
        res
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlimContest {
    pub region: String,
    pub title: String,
    pub introduction: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub seal_time: Option<NaiveDateTime>,
    pub state: String,
    pub is_registered: bool,
    pub need_pass: bool,
}

impl From<RawContest> for SlimContest {
    fn from(raw: RawContest) -> Self {
        let contest = Contest::from(raw);

        Self {
            region: contest.region,
            title: contest.title,
            introduction: contest.introduction,
            start_time: contest.start_time,
            end_time: contest.end_time,
            seal_time: contest.seal_time,
            state: contest.state,
            is_registered: false,
            need_pass: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestSettings {
    pub register_after_start: bool,
    pub unrate_after_start: bool,
    pub view_before_start: bool,
    pub view_after_end: bool,
    pub public_after_end: bool,
    pub submit_after_end: bool,
}

impl Default for ContestSettings {
    fn default() -> Self {
        Self {
            register_after_start: true,
            unrate_after_start: true,
            view_before_start: true,
            view_after_end: true,
            public_after_end: false,
            submit_after_end: true,
        }
    }
}

#[derive(PartialEq)]
pub enum ContestState {
    Preparing,
    Running,
    SealedRunning,
    Ended,
}

use std::fmt;
impl fmt::Display for ContestState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContestState::Preparing => f.write_str("Preparing"),
            ContestState::Running => f.write_str("Running"),
            ContestState::SealedRunning => f.write_str("SealedRunning"),
            ContestState::Ended => f.write_str("Ended"),
        }
    }
}

pub fn get_contest_state(contest: Contest, cur_time: NaiveDateTime) -> ContestState {
    if cur_time < contest.start_time {
        ContestState::Preparing
    } else {
        if let Some(seal_time) = contest.seal_time {
            if cur_time < seal_time {
                ContestState::Running
            } else {
                if let Some(end_time) = contest.end_time {
                    if cur_time < end_time {
                        ContestState::SealedRunning
                    } else {
                        ContestState::Ended
                    }
                } else {
                    ContestState::SealedRunning
                }
            }
        } else {
            if let Some(end_time) = contest.end_time {
                if cur_time < end_time {
                    ContestState::Running
                } else {
                    ContestState::Ended
                }
            } else {
                ContestState::Running
            }
        }
    }
}
