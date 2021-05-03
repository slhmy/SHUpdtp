use crate::errors::*;
use crate::utils::get_cur_naive_date_time;
use diesel::prelude::*;

pub fn have_access_setting(conn: &PgConnection, region: String) -> ServiceResult<bool> {
    use crate::schema::region_access_settings as region_access_settings_schema;

    if region_access_settings_schema::table
        .filter(region_access_settings_schema::region.eq(region))
        .count()
        .get_result::<i64>(conn)?
        > 0
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn check_acl(conn: &PgConnection, user_id: i32, region: String) -> ServiceResult<()> {
    use crate::schema::access_control_list as access_control_list_schema;

    if access_control_list_schema::table
        .filter(access_control_list_schema::user_id.eq(user_id))
        .filter(access_control_list_schema::region.eq(region))
        .count()
        .get_result::<i64>(conn)?
        == 1
    {
        Ok(())
    } else {
        let hint = "Not in ACL.".to_owned();
        Err(ServiceError::UnauthorizedWithHint(hint))
    }
}

// have right to get colume to see problem list
pub fn check_view_right(conn: &PgConnection, user_id: i32, region: String) -> ServiceResult<()> {
    use crate::schema::regions as regions_schema;
    let region_type = regions_schema::table
        .filter(regions_schema::name.eq(region.clone()))
        .select(regions_schema::self_type)
        .first::<String>(conn)?;
    if &region_type == "contest" {
        use crate::models::contests;
        use crate::schema::contests as contests_schema;

        let contest = contests::Contest::from(
            contests_schema::table
                .filter(contests_schema::region.eq(region.clone()))
                .first::<contests::RawContest>(conn)?,
        );

        use contests::ContestState::*;
        match contests::get_contest_state(contest.clone()) {
            Preparing => {
                if !contest.settings.view_before_start {
                    let hint = "Contest do not allows viewing before start.".to_owned();
                    return Err(ServiceError::UnauthorizedWithHint(hint));
                }
            }
            Ended => {
                if !contest.settings.view_after_end {
                    let hint = "Contest do not allows viewing after end.".to_owned();
                    return Err(ServiceError::UnauthorizedWithHint(hint));
                } else if contest.settings.public_after_end {
                    return Ok(());
                }
            }
            _ => (),
        }
    }
    check_acl(conn, user_id, region)
}

// have right to see problem detail and submit in region
pub fn check_solve_right(conn: &PgConnection, user_id: i32, region: String) -> ServiceResult<()> {
    use crate::schema::regions as regions_schema;
    let region_type = regions_schema::table
        .filter(regions_schema::name.eq(region.clone()))
        .select(regions_schema::self_type)
        .first::<String>(conn)?;
    if &region_type == "contest" {
        use crate::models::contests;
        use crate::schema::contests as contests_schema;

        let contest = contests::Contest::from(
            contests_schema::table
                .filter(contests_schema::region.eq(region.clone()))
                .first::<contests::RawContest>(conn)?,
        );

        use contests::ContestState::*;
        match contests::get_contest_state(contest.clone()) {
            Preparing => {
                let hint = "Contest do not allows visiting problems before start.".to_owned();
                return Err(ServiceError::UnauthorizedWithHint(hint));
            }
            Ended => {
                if !contest.settings.submit_after_end {
                    let hint = "Contest do not allows visiting problems before start.".to_owned();
                    return Err(ServiceError::UnauthorizedWithHint(hint));
                }
            }
            _ => (),
        }
    }
    check_acl(conn, user_id, region)
}
