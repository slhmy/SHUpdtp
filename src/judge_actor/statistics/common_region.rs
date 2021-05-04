use crate::models::*;
use mongodb::bson::doc;
use mongodb::sync::Database as MongoDB;
use std::collections::HashSet;

pub fn count_results(
    mongodb_database: MongoDB,
    submission: submissions::Submission,
    result_set: HashSet<String>,
) {
    // if not sample submission
    if let Some(region) = submission.region {
        if mongodb_database
            .collection("submission_statistics")
            .find_one(
                doc! {
                    "problem_id": submission.problem_id,
                    "region": region.clone(),
                },
                None,
            )
            .unwrap()
            .is_none()
        {
            mongodb_database
                .collection("submission_statistics")
                .insert_one(
                    doc! {
                        "problem_id": submission.problem_id,
                        "region": region.clone(),
                        "submit_times": 0,
                        "accept_times": 0,
                        "error_times": 0,
                        "avg_max_time": 0,
                        "avg_max_memory": 0,
                        "WRONG_ANSWER": 0,
                        "SUCCESS": 0,
                        "CPU_TIME_LIMIT_EXCEEDED": 0,
                        "REAL_TIME_LIMIT_EXCEEDED": 0,
                        "MEMORY_LIMIT_EXCEEDED": 0,
                        "RUNTIME_ERROR": 0,
                        "SYSTEM_ERROR": 0,
                        "UNKNOWN_ERROR": 0,
                    },
                    None,
                )
                .unwrap();
        }
        if let Some(doc) = mongodb_database
            .collection("submission_statistics")
            .find_one(
                doc! {
                    "problem_id": submission.problem_id,
                    "region": region.clone(),
                },
                None,
            )
            .unwrap()
        {
            mongodb_database
                .collection("submission_statistics")
                .update_one(
                    doc! {
                        "problem_id": submission.problem_id,
                        "region": region.clone(),
                    },
                    doc! {
                        "problem_id": submission.problem_id,
                        "region": region,
                        "submit_times": doc.get("submit_times").unwrap().as_i32().unwrap() + 1,
                        "accept_times": doc.get("accept_times").unwrap().as_i32().unwrap()
                            + match submission.is_accepted {
                                Some(is_accepted) => {
                                    if is_accepted { 1 } else { 0 }
                                },
                                None => 0
                            },
                        "error_times": doc.get("error_times").unwrap().as_i32().unwrap()
                            + match submission.is_accepted {
                                Some(_) => { 0 },
                                None => 1
                            },
                        "avg_max_time": 
                            match submission.is_accepted {
                                Some(_) => {
                                    let accept_times = doc.get("accept_times").unwrap().as_i32().unwrap();
                                    (doc.get("avg_max_time").unwrap().as_i32().unwrap()
                                    * accept_times
                                    + submission.max_time.unwrap())
                                    / (accept_times + 1)
                                },
                                None => doc.get("avg_max_time").unwrap().as_i32().unwrap()
                            },
                        "avg_max_memory": 
                            match submission.is_accepted {
                                Some(_) => {
                                    let accept_times = doc.get("accept_times").unwrap().as_i32().unwrap();
                                    (doc.get("avg_max_memory").unwrap().as_i32().unwrap()
                                    * accept_times
                                    + submission.max_memory.unwrap())
                                    / (accept_times + 1)
                                },
                                None => doc.get("avg_max_memory").unwrap().as_i32().unwrap()
                            },
                        "WRONG_ANSWER": if result_set.contains("WRONG_ANSWER") {
                            doc.get("WRONG_ANSWER").unwrap().as_i32().unwrap() + 1
                        } else {
                            doc.get("WRONG_ANSWER").unwrap().as_i32().unwrap() 
                        },
                        "SUCCESS": if result_set.contains("SUCCESS") {
                            doc.get("SUCCESS").unwrap().as_i32().unwrap() + 1
                        } else {
                            doc.get("SUCCESS").unwrap().as_i32().unwrap() 
                        },
                        "CPU_TIME_LIMIT_EXCEEDED": if result_set.contains("CPU_TIME_LIMIT_EXCEEDED") {
                            doc.get("CPU_TIME_LIMIT_EXCEEDED").unwrap().as_i32().unwrap() + 1
                        } else {
                            doc.get("CPU_TIME_LIMIT_EXCEEDED").unwrap().as_i32().unwrap() 
                        },
                        "REAL_TIME_LIMIT_EXCEEDED": if result_set.contains("REAL_TIME_LIMIT_EXCEEDED") {
                            doc.get("REAL_TIME_LIMIT_EXCEEDED").unwrap().as_i32().unwrap() + 1
                        } else {
                            doc.get("REAL_TIME_LIMIT_EXCEEDED").unwrap().as_i32().unwrap() 
                        },
                        "MEMORY_LIMIT_EXCEEDED": if result_set.contains("MEMORY_LIMIT_EXCEEDED") {
                            doc.get("MEMORY_LIMIT_EXCEEDED").unwrap().as_i32().unwrap() + 1
                        } else {
                            doc.get("MEMORY_LIMIT_EXCEEDED").unwrap().as_i32().unwrap() 
                        },
                        "RUNTIME_ERROR": if result_set.contains("RUNTIME_ERROR") {
                            doc.get("RUNTIME_ERROR").unwrap().as_i32().unwrap() + 1
                        } else {
                            doc.get("RUNTIME_ERROR").unwrap().as_i32().unwrap() 
                        },
                        "SYSTEM_ERROR": if result_set.contains("SYSTEM_ERROR") {
                            doc.get("SYSTEM_ERROR").unwrap().as_i32().unwrap() + 1
                        } else {
                            doc.get("SYSTEM_ERROR").unwrap().as_i32().unwrap() 
                        },
                        "UNKNOWN_ERROR": if result_set.contains("UNKNOWN_ERROR") {
                            doc.get("UNKNOWN_ERROR").unwrap().as_i32().unwrap() + 1
                        } else {
                            doc.get("UNKNOWN_ERROR").unwrap().as_i32().unwrap() 
                        },
                    },
                    None,
                )
                .unwrap();
        }
    }
}
