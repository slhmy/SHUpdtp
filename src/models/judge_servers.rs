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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutJudgeServerInfo {
    pub judger_version: String,
    pub hostname: String,
    pub cpu_core: i32,
    pub memory: f32,
    pub cpu: f32,
    pub task_number: i32,
    pub service_url: String,
    pub token: String,
    pub heartbeat_time: SystemTime,
    pub last_heartbeat: i32,
    pub is_deprecated: bool,
}

impl From<JudgeServerInfo> for OutJudgeServerInfo {
    fn from(raw: JudgeServerInfo) -> Self {
        Self {
            judger_version: raw.judger_version,
            hostname: raw.hostname,
            cpu_core: raw.cpu_core,
            memory: raw.memory,
            cpu: raw.cpu,
            task_number: raw.task_number,
            service_url: raw.service_url,
            token: raw.token,
            heartbeat_time: raw.heartbeat_time,
            last_heartbeat: raw.heartbeat_time.elapsed().unwrap().as_secs() as i32,
            is_deprecated: raw.is_deprecated,
        }
    }
}
