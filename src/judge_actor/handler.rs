use super::utils::*;
use super::JudgeActor;
use crate::models::*;
use crate::statics::JUDGE_SERVER_INFOS;
use crate::statics::WAITING_QUEUE;
use crate::utils::*;
use actix::prelude::*;
use diesel::prelude::*;
use mongodb::bson::doc;

#[derive(Debug, Clone, Deserialize)]
pub struct StartJudge();

impl Message for StartJudge {
    type Result = ();
}

impl Handler<StartJudge> for JudgeActor {
    type Result = ();

    fn handle(&mut self, _msg: StartJudge, _: &mut Self::Context) -> Self::Result {
        use crate::schema::submissions as submissions_schema;

        let mut queue_size = {
            let lock = WAITING_QUEUE.read().unwrap();
            lock.len()
        };
        info!("queue_size: {}", queue_size);
        while queue_size != 0 {
            let server = choose_judge_server();
            if server.is_none() {
                return ();
            }
            let (server_url, server_token) = server.unwrap();

            let task_uuid = {
                let mut lock = WAITING_QUEUE.write().unwrap();
                lock.pop_front().clone().unwrap()
            };

            let cur_state = submissions_schema::table
                .filter(submissions_schema::id.eq(task_uuid))
                .select(submissions_schema::state)
                .first::<String>(&self.db_connection)
                .expect("Error loading setting_data from submissions.");

            if cur_state == *"Waiting" {
                let setting_string = submissions_schema::table
                    .filter(submissions_schema::id.eq(task_uuid))
                    .select(submissions_schema::settings)
                    .first::<String>(&self.db_connection)
                    .expect("Error loading setting_data from submissions.");

                let target = submissions_schema::table.filter(submissions_schema::id.eq(task_uuid));
                diesel::update(target)
                    .set((submissions_schema::state.eq("Pending".to_owned()),))
                    .execute(&self.db_connection)
                    .expect("Error changing submissions's state to Pending.");

                info!("sending request to {}", server_url);
                {
                    let mut lock = JUDGE_SERVER_INFOS.write().unwrap();
                    let mut server_info = lock.get_mut(&server_url).unwrap();
                    server_info.task_number += 1;
                }
                let result_string =
                    run_judge_client(server_token, server_url.clone(), setting_string);
                info!("{}", result_string);

                {
                    let mut lock = JUDGE_SERVER_INFOS.write().unwrap();
                    let mut server_info = lock.get_mut(&server_url).unwrap();
                    server_info.task_number -= 1;
                }

                if result_string == *"" {
                    let target =
                        submissions_schema::table.filter(submissions_schema::id.eq(task_uuid));
                    diesel::update(target)
                        .set((submissions_schema::state.eq("Waiting".to_owned()),))
                        .execute(&self.db_connection)
                        .expect("Error changing submissions's state to Waiting.");

                    {
                        let mut lock = WAITING_QUEUE.write().unwrap();
                        lock.push_front(task_uuid);
                    }

                    info!("pushed {} back to queue", task_uuid);
                    continue;
                }

                let raw_result =
                    serde_json::from_str::<submissions::RawJudgeResult>(&result_string).unwrap();
                let result = submissions::JudgeResult::from(raw_result);

                // update submissions
                let target = submissions_schema::table.filter(submissions_schema::id.eq(task_uuid));
                diesel::update(target)
                    .set((
                        submissions_schema::state.eq("Finished".to_owned()),
                        submissions_schema::result.eq(serde_json::to_string(&result).unwrap()),
                        submissions_schema::is_accepted.eq(result.is_accepted),
                        submissions_schema::finish_time.eq(get_cur_naive_date_time()),
                        submissions_schema::max_time.eq(result.max_time),
                        submissions_schema::max_memory.eq(result.max_memory),
                        submissions_schema::err.eq(result.err),
                    ))
                    .execute(&self.db_connection)
                    .expect("Error changing submissions's data.");

                let submission = submissions::Submission::from(
                    submissions_schema::table
                        .filter(submissions_schema::id.eq(task_uuid))
                        .first::<submissions::RawSubmission>(&self.db_connection)
                        .unwrap(),
                );

                // if not sample submission
                if let Some(region) = submission.region {
                    if self
                        .mongodb_database
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
                        self.mongodb_database
                            .collection("submission_statistics")
                            .insert_one(
                                doc! {
                                    "problem_id": submission.problem_id,
                                    "region": region.clone(),
                                    "submit_times": 0,
                                    "accept_times": 0,
                                    "error_times": 0,
                                    "max_time": 0,
                                    "max_memory": 0,
                                },
                                None,
                            )
                            .unwrap();
                    }
                    if let Some(doc) = self
                        .mongodb_database
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
                        self.mongodb_database
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
                                    "max_time": 
                                        match submission.is_accepted {
                                            Some(_) => {
                                                let accept_times = doc.get("accept_times").unwrap().as_i32().unwrap();
                                                (doc.get("max_time").unwrap().as_i32().unwrap()
                                                * accept_times
                                                + submission.max_time.unwrap())
                                                / (accept_times + 1)
                                            },
                                            None => doc.get("max_time").unwrap().as_i32().unwrap()
                                        },
                                    "max_memory": 
                                        match submission.is_accepted {
                                            Some(_) => {
                                                let accept_times = doc.get("accept_times").unwrap().as_i32().unwrap();
                                                (doc.get("max_memory").unwrap().as_i32().unwrap()
                                                * accept_times
                                                + submission.max_memory.unwrap())
                                                / (accept_times + 1)
                                            },
                                            None => doc.get("max_memory").unwrap().as_i32().unwrap()
                                        }
                                },
                                None,
                            )
                            .unwrap();
                    }
                }
            }

            queue_size = {
                let lock = WAITING_QUEUE.read().unwrap();
                lock.len()
            };
        }

        ()
    }
}
