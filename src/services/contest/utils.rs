use server_core::errors::{ServiceError, ServiceResult};
use crate::models::contests::*;

pub fn check_settings_legal(settings: ContestSettings) -> ServiceResult<()> {
    if !settings.view_after_end && settings.public_after_end {
        let hint = "Can not allow public_after_end if view_after end is true".to_owned();
        return Err(ServiceError::BadRequest(hint));
    }
    Ok(())
}
