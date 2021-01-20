use actix_web::{web, HttpResponse, HttpRequest, get, post, put, delete};
use crate::errors::ServiceError;
use crate::services::judge_server::*;
use crate::judge_actor::JudgeActorAddr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HeartbeatBody {
    pub judger_version: String,
    pub hostname: String,
    pub cpu_core: i32,
    pub memory: f32,
    pub cpu: f32,
    pub service_url: Option<String>
}

#[derive(Debug, Clone, Serialize)]
struct HeartbeatResponse {
    data: String,
    error: Option<String>,
}

#[post("/heartbeat")]
pub async fn handle_heartbeat(
    body: web::Json<HeartbeatBody>,
    req: HttpRequest,
    judge_actor: web::Data<JudgeActorAddr>
) -> Result<HttpResponse, ServiceError> {
    let token = req.headers().get("x-judge-server-token").unwrap()
        .to_str().unwrap().to_string();

    record_server_info(
        body.judger_version.clone(),
        body.hostname.clone(),
        body.cpu_core,
        body.memory,
        body.cpu,
        body.service_url.clone(),
        token.clone(),
        judge_actor,
    ).await?;

    Ok(
        HttpResponse::Ok()
            .set_header("X-Judge-Server-Token", token)
            .set_header("Content-Type", "application/json")
            .json(HeartbeatResponse {
                data: "success".to_owned(),
                error: None,
            }
        )
    )
}