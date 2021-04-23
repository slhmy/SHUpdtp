use super::submissions;
use crate::schema::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable)]
pub struct RawSample {
    pub submission_id: Uuid,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "samples"]
pub struct InsertableSample {
    pub submission_id: Uuid,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub submission_id: Uuid,
    pub description: Option<String>,
    pub submission: submissions::Submission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlimSample {
    pub submission_id: Uuid,
    pub problem_id: i32,
    pub language: Option<String>,
    pub description: Option<String>,
    pub submission_state: String,
    pub is_accepted: Option<bool>,
    pub submit_time: NaiveDateTime,
    pub err: Option<String>,
}

impl From<Sample> for SlimSample {
    fn from(raw: Sample) -> Self {
        Self {
            submission_id: raw.submission_id,
            problem_id: raw.submission.problem_id,
            language: raw.submission.language,
            description: raw.description,
            submission_state: raw.submission.state,
            is_accepted: raw.submission.is_accepted,
            submit_time: raw.submission.submit_time,
            err: raw.submission.err,
        }
    }
}
