mod utils;

use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::problems::*;
use crate::models::utils::SizedList;
use actix_web::web;
use diesel::prelude::*;
use std::fs;
use std::io::prelude::*;
use std::process::Command;
use uuid::Uuid;

pub fn batch_create(
    zip_buf: &[u8],
    pool: web::Data<Pool>,
) -> ServiceResult<Vec<CreateProblemsResult>> {
    let tmp_folder = String::from("data/tmp/") + &Uuid::new_v4().to_hyphenated().to_string();
    let file_path = tmp_folder.clone() + "/raw.zip";
    fs::create_dir_all(&tmp_folder)?;

    let mut file = fs::File::create(file_path.clone())?;
    file.write_all(zip_buf).expect("Error writing zip.");

    let mut p = Command::new("unzip")
        .args(&["-o", &file_path, "-d", &tmp_folder])
        .spawn()?;
    p.wait()?;

    fs::remove_file(file_path)?;

    let mut res = Vec::new();

    for entry in fs::read_dir(tmp_folder.clone())? {
        let dir = entry?;

        match utils::read_insertable_problem(&dir.path().into_os_string().into_string().unwrap()) {
            Ok(insertable_problem) => {
                let mut target_problem = insertable_problem.clone();
                let mut settings: ProblemSettings =
                    serde_json::from_str(&insertable_problem.settings).unwrap();
                match utils::prepare_test_cases(
                    &(dir.path().into_os_string().into_string().unwrap() + "/TestCases"),
                    settings.is_spj,
                ) {
                    Ok(test_case_count) => {
                        settings.test_case_count = Some(test_case_count);
                        target_problem.settings = serde_json::to_string(&settings).unwrap();
                        info!("{:?}", target_problem);

                        let conn = &db_connection(&pool)?;

                        use crate::schema::problems as problems_schema;
                        match diesel::insert_into(problems_schema::table)
                            .values(&target_problem.clone())
                            .execute(conn)
                        {
                            Ok(_) => {
                                let id: i32 = problems_schema::table
                                    .filter(problems_schema::title.eq(target_problem.title))
                                    .select(problems_schema::id)
                                    .first(conn)?;

                                fs::remove_dir_all(format!("data/test_cases/{}", id)).unwrap_or({});

                                fs::rename(
                                    &(dir.path().into_os_string().into_string().unwrap()
                                        + "/TestCases"),
                                    format!("data/test_cases/{}", id),
                                )?;

                                res.push(CreateProblemsResult {
                                    title: dir.file_name().to_str().unwrap().to_owned(),
                                    is_success: true,
                                    id: Some(id),
                                });
                            }
                            Err(_) => {
                                let max_id: i32 = problems_schema::table
                                    .select(problems_schema::id)
                                    .order(problems_schema::id.desc())
                                    .first(conn)?;

                                diesel::sql_query(format!(
                                    "ALTER SEQUENCE problems_id_seq RESTART WITH {}",
                                    max_id + 1
                                ))
                                .execute(conn)?;

                                res.push(CreateProblemsResult {
                                    title: dir.file_name().to_str().unwrap().to_owned(),
                                    is_success: false,
                                    id: None,
                                });
                            }
                        }
                    }
                    Err(_) => {
                        res.push(CreateProblemsResult {
                            title: dir.file_name().to_str().unwrap().to_owned(),
                            is_success: false,
                            id: None,
                        });
                    }
                }
            }
            Err(_) => {
                res.push(CreateProblemsResult {
                    title: dir.file_name().to_str().unwrap().to_owned(),
                    is_success: false,
                    id: None,
                });
            }
        }
    }

    fs::remove_dir_all(&tmp_folder)?;

    Ok(res)
}

pub fn change_release_state(
    id: i32,
    target_state: bool,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    if !target_state {
        // do some check
    }

    let conn = &db_connection(&pool)?;

    use crate::schema::problems as problems_schema;
    diesel::update(problems_schema::table.filter(problems_schema::id.eq(id)))
        .set(problems_schema::is_released.eq(target_state))
        .execute(conn)?;

    Ok(())
}

pub fn get_list(
    id_filter: Option<i32>,
    title_filter: Option<String>,
    tag_filter: Option<Vec<String>>,
    difficulty_filter: Option<String>,
    release_filter: Option<bool>,
    id_order: Option<bool>,
    difficulty_order: Option<bool>,
    limit: i32,
    offset: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<SizedList<SlimProblem>> {
    let title_filter = if title_filter.is_none() {
        None
    } else {
        Some(String::from("%") + &title_filter.unwrap().as_str().replace(" ", "%") + "%")
    };

    let tag_filter: Vec<String> = if tag_filter.is_some() {
        if tag_filter.clone().unwrap().len() > 0 {
            tag_filter.unwrap()
        } else {
            Vec::<String>::new()
        }
    } else {
        Vec::<String>::new()
    };

    let (min_difficulty, max_difficulty) = if difficulty_filter.is_none() {
        (0.0, 10.0)
    } else {
        match difficulty_filter.unwrap().as_str() {
            "Navie" => (0.0, 2.5),
            "Easy" => (2.5, 5.0),
            "Middle" => (5.0, 7.5),
            "Hard" => (7.5, 10.0),
            _ => (0.0, 10.0),
        }
    };

    let conn = &db_connection(&pool)?;

    use crate::schema::problems as problems_schema;
    let target = problems_schema::table
        .filter(
            problems_schema::id
                .nullable()
                .eq(id_filter)
                .or(id_filter.is_none()),
        )
        .filter(
            problems_schema::tags
                .overlaps_with(tag_filter.clone())
                .or(tag_filter.is_empty()),
        )
        .filter(
            problems_schema::title
                .nullable()
                .like(title_filter.clone())
                .or(title_filter.is_none()),
        )
        .filter(
            problems_schema::is_released
                .eq(release_filter.unwrap_or_default())
                .or(release_filter.is_none()),
        )
        .filter(problems_schema::difficulty.between(min_difficulty, max_difficulty));

    let total: i64 = target.clone().count().get_result(conn)?;

    let target = target.offset(offset.into()).limit(limit.into());

    let problems: Vec<RawProblem> = match id_order {
        None => match difficulty_order {
            None => target.load(conn)?,
            Some(true) => target.order(problems_schema::difficulty.asc()).load(conn)?,
            Some(false) => target
                .order(problems_schema::difficulty.desc())
                .load(conn)?,
        },
        Some(true) => target.order(problems_schema::id.asc()).load(conn)?,
        Some(false) => target.order(problems_schema::id.desc()).load(conn)?,
    };

    let out_problems = {
        let mut res = Vec::new();
        for problem in problems {
            res.push(SlimProblem::from(problem));
        }
        res
    };

    Ok(SizedList {
        total: total,
        list: out_problems,
    })
}

pub fn get(id: i32, pool: web::Data<Pool>) -> ServiceResult<Problem> {
    let conn = &db_connection(&pool)?;

    use crate::schema::problems as problems_schema;

    let problem: RawProblem = problems_schema::table
        .filter(problems_schema::id.eq(id))
        .first(conn)?;

    Ok(Problem::from(problem))
}

pub fn delete(id: i32, pool: web::Data<Pool>) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    use crate::schema::problems as problems_schema;
    use crate::schema::samples as samples_schema;
    use crate::schema::submissions as submissions_schema;

    if problems_schema::table
        .filter(problems_schema::id.eq(id))
        .select(problems_schema::is_released)
        .first::<bool>(conn)?
    {
        let hint = "Problem is_released.".to_string();
        return Err(ServiceError::BadRequest(hint));
    }

    // find related samples and delete them all
    let submission_ids: Vec<Uuid> = samples_schema::table
        .inner_join(
            submissions_schema::table.on(samples_schema::submission_id.eq(submissions_schema::id)),
        )
        .filter(submissions_schema::problem_id.eq(id))
        .select(samples_schema::submission_id)
        .load(conn)?;
    diesel::delete(
        samples_schema::table.filter(samples_schema::submission_id.eq_any(submission_ids.clone())),
    )
    .execute(conn)?;
    diesel::delete(submissions_schema::table.filter(submissions_schema::id.eq_any(submission_ids)))
        .execute(conn)?;

    diesel::delete(problems_schema::table.filter(problems_schema::id.eq(id))).execute(conn)?;

    fs::remove_dir_all(&format!("data/test_cases/{}", id))?;

    let max_id: i32 = problems_schema::table
        .select(problems_schema::id)
        .order(problems_schema::id.desc())
        .first(conn)?;

    diesel::sql_query(format!(
        "ALTER SEQUENCE problems_id_seq RESTART WITH {}",
        max_id + 1
    ))
    .execute(conn)?;

    Ok(())
}
