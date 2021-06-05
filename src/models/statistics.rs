use crate::models::submissions::*;
use crate::statics::RESULT_STATISTICS_CACHE;
use diesel::prelude::*;
use server_core::database::*;
use server_core::errors::ServiceResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionStatistics {
    pub problem_id: i32,
    pub region: String,
    pub submit_times: i32,
    pub accept_times: i32,
    pub error_times: i32,
    pub avg_max_time: i32,
    pub avg_max_memory: i32,
    pub result_count: ResultCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultCount {
    pub wrong_answer: i32,
    pub success: i32,
    pub cpu_time_limit_exceeded: i32,
    pub real_time_limit_exceeded: i32,
    pub memory_limit_exceeded: i32,
    pub runtime_error: i32,
    pub system_error: i32,
    pub unknown_error: i32,
}

pub fn get_results(
    conn: &PooledConnection,
    region: String,
    problem_id: i32,
) -> ServiceResult<SubmissionStatistics> {
    let has_cache = {
        let result_statistics = RESULT_STATISTICS_CACHE.read().unwrap();
        result_statistics
            .get(&(region.clone(), problem_id))
            .is_some()
    };

    if !has_cache {
        count_results(conn, &region, problem_id)?;
    }

    Ok({
        let result_statistics = RESULT_STATISTICS_CACHE.read().unwrap();
        result_statistics
            .get(&(region.clone(), problem_id))
            .unwrap()
            .to_owned()
    })
}

fn count_results(conn: &PooledConnection, region: &str, problem_id: i32) -> ServiceResult<()> {
    let mut statistics = SubmissionStatistics {
        problem_id,
        region: region.to_owned(),
        submit_times: 0,
        accept_times: 0,
        error_times: 0,
        avg_max_time: 0,
        avg_max_memory: 0,
        result_count: ResultCount {
            wrong_answer: 0,
            success: 0,
            cpu_time_limit_exceeded: 0,
            real_time_limit_exceeded: 0,
            memory_limit_exceeded: 0,
            runtime_error: 0,
            system_error: 0,
            unknown_error: 0,
        },
    };

    use crate::schema::submissions as submissions_schema;
    let raw_submissions: Vec<RawSubmission> = submissions_schema::table
        .filter(submissions_schema::region.eq(region.to_string()))
        .filter(submissions_schema::problem_id.eq(problem_id))
        .load(conn)?;

    for raw_submission in raw_submissions {
        let submission = Submission::from(raw_submission);
        update_submission_statistics(&mut statistics, submission);
    }

    {
        let mut result_statistics = RESULT_STATISTICS_CACHE.write().unwrap();
        result_statistics.insert((region.to_owned(), problem_id), statistics);
    }

    Ok(())
}

fn update_submission_statistics(statistics: &mut SubmissionStatistics, submission: Submission) {
    if let Some(result) = submission.result {
        if let Some(is_accepted) = result.is_accepted {
            if is_accepted {
                statistics.accept_times += 1;
            }
        }

        if let Some(_) = result.err {
            statistics.error_times += 1;
        }

        if let Some(max_time) = result.max_time {
            let effective_time = statistics.submit_times - statistics.error_times;
            statistics.avg_max_time =
                (statistics.avg_max_time * effective_time + max_time) / (effective_time + 1);
        }

        if let Some(max_memory) = result.max_memory {
            let effective_time = statistics.submit_times - statistics.error_times;
            statistics.avg_max_memory =
                (statistics.avg_max_memory * effective_time + max_memory) / (effective_time + 1);
        }

        statistics.submit_times += 1;
    }

    if let Some(result_set) = submission.out_results {
        for result in result_set {
            match result.as_str() {
                "WRONG_ANSWER" => {
                    statistics.result_count.wrong_answer += 1;
                }
                "SUCCESS" => {
                    statistics.result_count.success += 1;
                }
                "CPU_TIME_LIMIT_EXCEEDED" => {
                    statistics.result_count.cpu_time_limit_exceeded += 1;
                }
                "REAL_TIME_LIMIT_EXCEEDED" => {
                    statistics.result_count.real_time_limit_exceeded += 1;
                }
                "MEMORY_LIMIT_EXCEEDED" => {
                    statistics.result_count.memory_limit_exceeded += 1;
                }
                "RUNTIME_ERROR" => {
                    statistics.result_count.runtime_error += 1;
                }
                "SYSTEM_ERROR" => {
                    statistics.result_count.system_error += 1;
                }
                "UNKNOWN_ERROR" => {
                    statistics.result_count.unknown_error += 1;
                }
                _ => {}
            }
        }
    }
}
