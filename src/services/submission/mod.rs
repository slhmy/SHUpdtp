use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::judge_actor::{handler::StartJudge, JudgeActorAddr};
use crate::models::utils::SizedList;
use crate::models::*;
use crate::statics::WAITING_QUEUE;
use crate::utils::get_cur_naive_date_time;
use actix_web::web;
use diesel::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

pub fn create(
    region: Option<String>,
    problem_id: i32,
    user_id: i32,
    src: String,
    language: String,
    pool: web::Data<Pool>,
    judge_actor: web::Data<JudgeActorAddr>,
) -> ServiceResult<Uuid> {
    let id = Uuid::new_v4();
    let language_config = languages::get_lang_config(&language);

    let conn = &db_connection(&pool)?;
    use crate::schema::problems as problems_schema;
    use crate::schema::submissions as submissions_schema;

    let raw_problem: problems::RawProblem = problems_schema::table
        .filter(problems_schema::id.eq(problem_id))
        .first(conn)?;
    let problem = problems::Problem::from(raw_problem);
    let mut spj_src = None;
    if problem.settings.is_spj {
        let mut file = File::open(format!("data/test_cases/{}/spj_src.cpp", problem.id))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        spj_src = Some(contents);
    }

    let settings = submissions::JudgeSettings {
        language_config: language_config,
        src: src,
        max_cpu_time: if &language == "c" || &language == "cpp" {
            problem.settings.high_performance_max_cpu_time
        } else {
            problem.settings.other_max_cpu_time
        },
        max_memory: if &language == "c" || &language == "cpp" {
            problem.settings.high_performance_max_memory
        } else {
            problem.settings.other_max_memory
        },
        test_case_id: Some(problem.id.to_string()),
        test_case: None,
        spj_version: Some("1".to_owned()),
        spj_config: if problem.settings.is_spj {
            Some(languages::spj_config())
        } else {
            None
        },
        spj_compile_config: if problem.settings.is_spj {
            Some(languages::spj_compile_config())
        } else {
            None
        },
        spj_src: spj_src,
        output: !problem.settings.opaque_output,
    };

    let settings_string = serde_json::to_string(&settings).unwrap();

    diesel::insert_into(submissions_schema::table)
        .values(&submissions::InsertableSubmission {
            id: id,
            problem_id: problem_id,
            region: region,
            user_id: user_id,
            state: String::from("Waiting"),
            settings: settings_string,
            result: None,
            submit_time: get_cur_naive_date_time(),
            is_accepted: None,
            finish_time: None,
            max_time: None,
            max_memory: None,
            language: Some(language),
            err: None,
        })
        .execute(conn)?;

    {
        let mut lock = WAITING_QUEUE.write().unwrap();
        lock.push_back(id);
    }

    judge_actor.addr.do_send(StartJudge());

    Ok(id)
}

pub fn get(id: Uuid, pool: web::Data<Pool>) -> ServiceResult<submissions::Submission> {
    let conn = &db_connection(&pool)?;

    use crate::schema::submissions as submissions_schema;

    let raw: submissions::RawSubmission = submissions_schema::table
        .filter(submissions_schema::id.eq(id))
        .first(conn)?;

    Ok(submissions::Submission::from(raw))
}

pub fn get_list(
    region_filter: Option<String>,
    problem_id_filter: Option<i32>,
    user_id_filter: Option<i32>,
    limit: i32,
    offset: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<SizedList<submissions::SlimSubmission>> {
    let conn = &db_connection(&pool)?;

    use crate::schema::submissions as submissions_schema;

    let target = submissions_schema::table
        .filter(
            submissions_schema::region
                .nullable()
                .eq(region_filter.clone())
                .or(region_filter.is_none()),
        )
        .filter(
            submissions_schema::problem_id
                .nullable()
                .eq(problem_id_filter)
                .or(problem_id_filter.is_none()),
        )
        .filter(
            submissions_schema::user_id
                .nullable()
                .eq(user_id_filter)
                .or(user_id_filter.is_none()),
        );

    let total: i64 = target.clone().count().get_result(conn)?;

    let raw_submissions: Vec<submissions::RawSubmission> = target
        .offset(offset.into())
        .limit(limit.into())
        .order(submissions_schema::submit_time.desc())
        .load(conn)?;

    let mut res = Vec::new();
    for raw_submission in raw_submissions {
        res.push(submissions::SlimSubmission::from(raw_submission));
    }

    Ok(SizedList {
        total: total,
        list: res,
    })
}
