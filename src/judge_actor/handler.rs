use super::statistics::*;
use super::utils::*;
use super::JudgeActor;
use crate::models::*;
use crate::services::rank::utils::update_acm_rank_cache;
use crate::services::region::utils::get_self_type;
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

            let cur_state = match submissions_schema::table
                .filter(submissions_schema::id.eq(task_uuid))
                .select(submissions_schema::state)
                .first::<String>(&self.db_connection)
            {
                Ok(cur_state) => cur_state,
                Err(_) => {
                    log::error!("Error loading setting_data from submissions.");
                    return;
                }
            };

            if cur_state == *"Waiting" {
                // run judge
                let setting_string = match submissions_schema::table
                    .filter(submissions_schema::id.eq(task_uuid))
                    .select(submissions_schema::settings)
                    .first::<String>(&self.db_connection)
                {
                    Ok(setting_string) => setting_string,
                    Err(_) => {
                        log::error!("Error loading setting_data from submissions.");
                        return;
                    }
                };

                let target = submissions_schema::table.filter(submissions_schema::id.eq(task_uuid));
                match diesel::update(target)
                    .set((submissions_schema::state.eq("Pending".to_owned()),))
                    .execute(&self.db_connection)
                {
                    Ok(_) => (),
                    Err(_) => {
                        log::error!("Error changing submissions's state to Pending.");
                        return;
                    }
                };

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
                    match diesel::update(target)
                        .set((submissions_schema::state.eq("Waiting".to_owned()),))
                        .execute(&self.db_connection)
                    {
                        Ok(_) => (),
                        Err(_) => {
                            log::error!("Error changing submissions's state to Waiting.");
                            return;
                        }
                    };

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
                let mut result_set = std::collections::HashSet::new();
                match diesel::update(target)
                    .set((
                        submissions_schema::state.eq("Finished".to_owned()),
                        submissions_schema::result.eq(serde_json::to_string(&result).unwrap()),
                        submissions_schema::is_accepted.eq(result.is_accepted),
                        submissions_schema::finish_time.eq(get_cur_naive_date_time()),
                        submissions_schema::max_time.eq(result.max_time),
                        submissions_schema::max_memory.eq(result.max_memory),
                        submissions_schema::err.eq(result.err),
                        submissions_schema::out_results.eq({
                            if let Some(details) = result.details {
                                let mut res = Vec::new();
                                for detail in details {
                                    result_set.insert(detail.result);
                                }
                                for e in result_set.clone() {
                                    res.push(e);
                                }
                                Some(res)
                            } else {
                                None
                            }
                        }),
                    ))
                    .execute(&self.db_connection)
                {
                    Ok(_) => (),
                    Err(_) => {
                        log::error!("Error changing submissions's data.");
                        return;
                    }
                };

                let submission = submissions::Submission::from(
                    match submissions_schema::table
                        .filter(submissions_schema::id.eq(task_uuid))
                        .first::<submissions::RawSubmission>(&self.db_connection)
                    {
                        Ok(raw_submission) => raw_submission,
                        Err(_) => {
                            log::error!("Error querying submission.");
                            return;
                        }
                    }
                );

                common_region::count_results(
                    self.mongodb_database.clone(),
                    submission.clone(),
                    result_set.clone(),
                );

                if let Some(region) = submission.region.clone() {
                    if match get_self_type(region, &self.db_connection)
                    {
                        Ok(region_type) => region_type,
                        Err(_) => {
                            log::error!("Error getting region type.");
                            return;
                        }
                    } == "contest" {
                        match update_acm_rank_cache(
                            submission.region.unwrap(),
                            &self.db_connection,
                            false,
                        )
                        {
                            Ok(_) => (),
                            Err(_) => {
                                log::error!("Error updating acm rank cache.");
                                return;
                            }
                        };
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
