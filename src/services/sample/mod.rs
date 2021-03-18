use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::judge_actor::JudgeActorAddr;
use crate::models::*;
use crate::services::submission;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

pub fn create(
    problem_id: i32,
    user_id: i32,
    src: String,
    language: String,
    description: Option<String>,
    pool: web::Data<Pool>,
    judge_actor: web::Data<JudgeActorAddr>,
) -> ServiceResult<Uuid> {
    let submission_id = submission::create(
        None,
        problem_id,
        user_id,
        src,
        language,
        pool.clone(),
        judge_actor,
    )?;

    let conn = &db_connection(&pool)?;
    use crate::schema::samples as samples_schema;

    diesel::insert_into(samples_schema::table)
        .values(&samples::InsertableSample {
            submission_id: submission_id,
            description: description,
        })
        .execute(conn)?;

    Ok(submission_id)
}

pub fn get_list(
    description_filter: Option<String>,
    limit: i32,
    offset: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<Vec<samples::SlimSample>> {
    let description_filter = if description_filter.is_none() {
        None
    } else {
        Some(String::from("%") + &description_filter.unwrap().as_str().replace(" ", "%") + "%")
    };

    let conn = &db_connection(&pool)?;

    use crate::schema::samples as samples_schema;
    use crate::schema::submissions as submissions_schema;

    let raw: Vec<(samples::RawSample, submissions::RawSubmission)> = samples_schema::table
        .filter(
            samples_schema::description
                .nullable()
                .like(description_filter.clone())
                .or(description_filter.is_none()),
        )
        .inner_join(
            submissions_schema::table.on(samples_schema::submission_id.eq(submissions_schema::id)),
        )
        .limit(limit.into())
        .offset(offset.into())
        .order(submissions_schema::submit_time.desc())
        .load(conn)?;

    let mut res = Vec::new();
    for (raw_sample, raw_submission) in raw {
        let slim_sample = samples::SlimSample::from(samples::Sample {
            submission_id: raw_sample.submission_id,
            description: raw_sample.description,
            submission: submissions::Submission::from(raw_submission),
        });
        res.push(slim_sample);
    }

    Ok(res)
}

pub fn get(id: Uuid, pool: web::Data<Pool>) -> ServiceResult<samples::Sample> {
    let conn = &db_connection(&pool)?;

    use crate::schema::samples as samples_schema;
    use crate::schema::submissions as submissions_schema;

    let (raw_sample, raw_submission): (samples::RawSample, submissions::RawSubmission) =
        samples_schema::table
            .filter(samples_schema::submission_id.eq(id))
            .inner_join(
                submissions_schema::table
                    .on(samples_schema::submission_id.eq(submissions_schema::id)),
            )
            .first(conn)?;

    Ok(samples::Sample {
        submission_id: raw_sample.submission_id,
        description: raw_sample.description,
        submission: submissions::Submission::from(raw_submission),
    })
}

pub fn delete(id: Uuid, pool: web::Data<Pool>) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    use crate::schema::samples as samples_schema;
    use crate::schema::submissions as submissions_schema;

    diesel::delete(samples_schema::table.filter(samples_schema::submission_id.eq(id)))
        .execute(conn)?;

    diesel::delete(submissions_schema::table.filter(submissions_schema::id.eq(id)))
        .execute(conn)?;

    Ok(())
}
