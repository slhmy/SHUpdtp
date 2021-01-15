use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct JudgeServerInfo {
    pub judger_version: String,
    pub hostname: String,
    pub cpu_core: i32,
    pub memory: f32,
    pub cpu: f32,
    pub task_number: i32,
    pub service_url: String,
    pub token: String,
    pub heartbeat_time: SystemTime,
    pub is_deprecated: bool,
}