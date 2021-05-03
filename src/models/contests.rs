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
pub struct Contest {
    pub region: String,
    pub title: String,
    pub introduction: Option<String>,
    pub start_time: Option<NaiveDateTime>,
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
        res.state = format!("{}", get_contest_state(res.clone()));
        res
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlimContest {
    pub region: String,
    pub title: String,
    pub introduction: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub seal_time: Option<NaiveDateTime>,
    pub state: String,
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

pub fn is_settings_legal(settings: ContestSettings) -> bool {
    if !settings.view_after_end && settings.public_after_end {
        return false;
    }
    true
}

pub fn get_contest_state(contest: Contest) -> ContestState {
    use crate::utils::get_cur_naive_date_time;
    let cur_time = get_cur_naive_date_time();
    if let Some(start_time) = contest.start_time {
        if cur_time < start_time {
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
