use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::judge_actor::JudgeActorAddr;
use crate::models::utils::SizedList;
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
    problem_id_filter: Option<i32>,
    language_filter: Option<String>,
    submit_time_order: Option<bool>,
    limit: i32,
    offset: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<SizedList<samples::SlimSample>> {
    let description_filter = if let Some(inner_data) = description_filter {
        Some(String::from("%") + &inner_data.as_str().replace(" ", "%") + "%")
    } else {
        None
    };

    let conn = &db_connection(&pool)?;

    use crate::schema::samples as samples_schema;
    use crate::schema::submissions as submissions_schema;

    let target = samples_schema::table
        .inner_join(
            submissions_schema::table.on(samples_schema::submission_id.eq(submissions_schema::id)),
        )
        .filter(
            samples_schema::description
                .nullable()
                .like(description_filter.clone())
                .or(description_filter.is_none()),
        )
        .filter(
            submissions_schema::problem_id
                .nullable()
                .eq(problem_id_filter)
                .or(problem_id_filter.is_none()),
        )
        .filter(
            submissions_schema::language
                .nullable()
                .eq(language_filter.clone())
                .or(language_filter.is_none()),
        );

    let total: i64 = target.clone().count().get_result(conn)?;

    let target = target.offset(offset.into()).limit(limit.into());

    let raw: Vec<(samples::RawSample, submissions::RawSubmission)> = match submit_time_order {
        None => target
            .order(submissions_schema::submit_time.desc())
            .load(conn)?,
        Some(true) => target
            .order(submissions_schema::submit_time.asc())
            .load(conn)?,
        Some(false) => target
            .order(submissions_schema::submit_time.desc())
            .load(conn)?,
    };

    let mut res = Vec::new();
    for (raw_sample, raw_submission) in raw {
        let slim_sample = samples::SlimSample::from(samples::Sample {
            submission_id: raw_sample.submission_id,
            description: raw_sample.description,
            submission: submissions::Submission::from(raw_submission),
        });
        res.push(slim_sample);
    }

    Ok(SizedList {
        total: total,
        list: res,
    })
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
