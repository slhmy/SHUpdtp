use crate::models::{judge_servers::JudgeServerInfo, ranks::ACMRank, users::AuthConfig};
use regex::Regex;
use std::io::Read;
use std::{
    collections::{HashMap, VecDeque},
    sync::RwLock,
};
use uuid::Uuid;

lazy_static! {
    pub static ref WAITING_QUEUE: RwLock<VecDeque::<Uuid>> = RwLock::new(VecDeque::new());
    pub static ref ACM_RANK_CACHE: RwLock<HashMap<String, ACMRank>> = RwLock::new(HashMap::new());
    pub static ref JUDGE_SERVER_INFOS: RwLock<HashMap<String, JudgeServerInfo>> =
        RwLock::new(HashMap::new());
    pub static ref RE_EMAIL: Regex =
        Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
    pub static ref RE_MOBILE: Regex =
        Regex::new(r"^((13[0-9])|(14[5|7])|(15([0-3]|[5-9]))|(18[0,5-9]))\d{8}$").unwrap();
    pub static ref RE_PASSWORD: Regex = Regex::new(r"^\S{6,20}$").unwrap();
    pub static ref AUTH_CONFIG: HashMap<String, AuthConfig> = {
        let mut file = std::fs::File::open("auth.toml").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    };
}
