use crate::errors::ServiceError;
use crate::judge_actor::JudgeActorAddr;
use crate::services::judge_server::*;
use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HeartbeatBody {
    pub judger_version: String,
    pub hostname: String,
    pub cpu_core: i32,
    pub memory: f32,
    pub cpu: f32,
    pub service_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct HeartbeatResponse {
    data: String,
    error: Option<String>,
}

pub async fn handle_heartbeat(
    body: web::Json<HeartbeatBody>,
    req: HttpRequest,
    judge_actor: web::Data<JudgeActorAddr>,
) -> Result<HttpResponse, ServiceError> {
    let token = req
        .headers()
        .get("x-judge-server-token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    record_server_info(
        body.judger_version.clone(),
        body.hostname.clone(),
        body.cpu_core,
        body.memory,
        body.cpu,
        body.service_url.clone(),
        token.clone(),
        judge_actor,
    )
    .await?;

    Ok(HttpResponse::Ok()
        .set_header("X-Judge-Server-Token", token)
        .set_header("Content-Type", "application/json")
        .json(HeartbeatResponse {
            data: "success".to_owned(),
            error: None,
        }))
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitRequestForm {
    pub problem_id: i32,
    pub problem_region: String,
    pub src: String,
    pub language: String,
    pub judge_type: String,
    pub output: bool,
}

pub async fn get_server_info(id: Identity) -> Result<HttpResponse, ServiceError> {
    server_info(id)
        .await
        .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn get_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("data/tmp/{}", sanitize_filename::sanitize(&filename));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}
