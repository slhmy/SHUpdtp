use crate::errors::ServiceResult;
use crate::judge_actor::{handler::StartJudge, JudgeActorAddr};
use crate::models::judge_servers::JudgeServerInfo;
use crate::statics::JUDGE_SERVER_INFOS;
use actix_web::client::Client;
use actix_web::web;
use std::time::SystemTime;

pub async fn record_server_info(
    judger_version: String,
    hostname: String,
    cpu_core: i32,
    memory: f32,
    cpu: f32,
    service_url: Option<String>,
    token: String,
    judge_actor: web::Data<JudgeActorAddr>,
) -> ServiceResult<()> {
    if !service_url.is_none() {
        let url = service_url.clone().unwrap();
        let task_number = {
            let lock = JUDGE_SERVER_INFOS.read().unwrap();
            if lock.get(&url).is_none() {
                0
            } else {
                let target = lock.get(&url).unwrap();
                target.task_number
            }
        };

        let response = Client::new()
            .post(format!("{}/ping", url))
            .set_header("X-Judge-Server-Token", token.clone())
            .set_header("Content-Type", "application/json")
            .send()
            .await;

        let is_deprecated = {
            if !response.is_ok() {
                info!("setting is_deprecated to true");
                true
            } else {
                false
            }
        };

        let now = SystemTime::now();
        let judge_server_info = JudgeServerInfo {
            judger_version: judger_version.clone(),
            hostname: hostname.clone(),
            cpu_core: cpu_core,
            memory: memory,
            cpu: cpu,
            task_number: task_number,
            service_url: url,
            token: token.clone(),
            heartbeat_time: now,
            is_deprecated: is_deprecated,
        };
        let mut lock = JUDGE_SERVER_INFOS.write().unwrap();
        lock.insert(service_url.clone().unwrap(), judge_server_info);

        if !is_deprecated {
            judge_actor.addr.do_send(StartJudge());
        }
    }

    Ok(())
}
