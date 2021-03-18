use super::utils::*;
use super::JudgeActor;
use crate::models::*;
use crate::statics::JUDGE_SERVER_INFOS;
use crate::statics::WAITING_QUEUE;
use crate::utils::*;
use actix::prelude::*;
use diesel::prelude::*;

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
            lock.len().clone()
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
                .first::<String>(&self.0)
                .expect("Error loading setting_data from submissions.");

            if cur_state == "Waiting".to_owned() {
                let setting_string = submissions_schema::table
                    .filter(submissions_schema::id.eq(task_uuid))
                    .select(submissions_schema::settings)
                    .first::<String>(&self.0)
                    .expect("Error loading setting_data from submissions.");

                let target = submissions_schema::table.filter(submissions_schema::id.eq(task_uuid));
                diesel::update(target)
                    .set((submissions_schema::state.eq("Pending".to_owned()),))
                    .execute(&self.0)
                    .expect("Error changing submissions's state to Pending.");

                info!("sending request to {}", server_url);
                {
                    let mut lock = JUDGE_SERVER_INFOS.write().unwrap();
                    let mut server_info = lock.get(&server_url).unwrap().clone();
                    server_info.task_number += 1;
                    lock.insert(server_url.clone(), server_info);
                }
                let result_string =
                    run_judge_client(server_token, server_url.clone(), setting_string);
                info!("{}", result_string);

                {
                    let mut lock = JUDGE_SERVER_INFOS.write().unwrap();
                    let mut server_info = lock.get(&server_url).unwrap().clone();
                    server_info.task_number -= 1;
                    lock.insert(server_url, server_info);
                }

                if result_string == String::from("") {
                    let target =
                        submissions_schema::table.filter(submissions_schema::id.eq(task_uuid));
                    diesel::update(target)
                        .set((submissions_schema::state.eq("Waiting".to_owned()),))
                        .execute(&self.0)
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
                    ))
                    .execute(&self.0)
                    .expect("Error changing submissions's data.");
            }

            queue_size = {
                let lock = WAITING_QUEUE.read().unwrap();
                lock.len().clone()
            };
        }

        ()
    }
}
