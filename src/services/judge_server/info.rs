use crate::models::judge_servers::OutJudgeServerInfo;
use crate::statics::JUDGE_SERVER_INFOS;
use actix_identity::Identity;
use server_core::errors::ServiceResult;

pub async fn server_info(_id: Identity) -> ServiceResult<Vec<OutJudgeServerInfo>> {
    let lock = JUDGE_SERVER_INFOS.read().unwrap();
    let mut info_vec: Vec<OutJudgeServerInfo> = Vec::new();
    for (_url, info) in lock.iter() {
        info_vec.push(OutJudgeServerInfo::from(info.clone()));
    }
    Ok(info_vec)
}
