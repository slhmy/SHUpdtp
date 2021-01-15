#[macro_use] extern crate serde_derive;

use actix_web::client::Client;
use actix_http::Error;
use std::io;
use std::str; 
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileConfig {
    pub src_name: String, 
    pub exe_name: String,
    pub max_cpu_time: i32,
    pub max_real_time: i32,
    pub max_memory: i32,
    pub compile_command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunConfig {
    pub command: String,
    pub seccomp_rule: Option<String>,
    pub env: Vec<String>,
    pub memory_limit_check_only: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub compile: CompileConfig,
    pub run: RunConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpjConfig {
    pub exe_name: String,
    pub command: String,
    pub seccomp_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpjCompileConfig {
    pub src_name: String,
    pub exe_name: String,
    pub max_cpu_time: i32,
    pub max_real_time: i32,
    pub max_memory: i32,
    pub compile_command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JudgeSetting {
    language_config: LanguageConfig,
    src: String,
    max_cpu_time: i32,
    max_memory: i32,
    test_case_id: Option<String>,
    test_case: Option<Vec<TestCase>>,
    spj_version: Option<String>,
    spj_config: Option<SpjConfig>,
    spj_compile_config: Option<SpjCompileConfig>,
    spj_src: Option<String>,
    output: bool,
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut token = String::new();
    stdin.read_line(&mut token)?;
    let mut url = String::new();
    stdin.read_line(&mut url)?;
    let mut judge_setting_string = String::new();
    stdin.read_line(&mut judge_setting_string)?;
    let judge_setting: JudgeSetting = serde_json::from_str(&judge_setting_string.trim())?;
    let time_out = (120) as u64;

    // Create request builder, configure request and send
    let mut response = Client::new()
        .post(format!("{}/judge", url.trim()))
        .set_header("X-Judge-Server-Token", token.trim())
        .set_header("Content-Type", "application/json")
        .timeout(Duration::new(time_out, 0))
        .send_json(&judge_setting)
        .await?;

    let result_vec = response.body().await?.to_vec();
    let result_str = str::from_utf8(&result_vec)?;
    println!("{}", result_str.trim());

    Ok(())
}