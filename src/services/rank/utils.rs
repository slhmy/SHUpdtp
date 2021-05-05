use crate::errors::ServiceResult;
use crate::models::access_control_list::*;
use crate::models::contests::*;
use crate::models::ranks::*;
use crate::models::region_links::*;
use crate::models::submissions::*;
use crate::services::region::utils::*;
use crate::statics::ACM_RANK_CACHE;
use crate::utils::get_cur_naive_date_time;
use diesel::prelude::*;

pub fn update_acm_rank_cache(
    region: String,
    conn: &PgConnection,
    is_final: bool,
) -> ServiceResult<()> {
    log::info!("Updating acm rank");
    if &get_self_type(region.clone(), conn)? != "contest" {
        return Ok(());
    }

    use crate::schema::contests as contests_schema;
    let contest = Contest::from(
        contests_schema::table
            .filter(contests_schema::region.eq(region.clone()))
            .first::<RawContest>(conn)?,
    );

    let mut rank = ACMRank {
        region: region.clone(),
        last_updated_time: get_cur_naive_date_time(),
        columns: Vec::new(),
    };

    use crate::schema::access_control_list as access_control_list_schema;

    let access_control_list: Vec<AccessControlListColumn> = access_control_list_schema::table
        .filter(access_control_list_schema::region.eq(region.clone()))
        .load(conn)?;

    for access_control_list_colunm in access_control_list {
        rank.columns.push(build_acm_rank_column(
            contest.clone(),
            access_control_list_colunm,
            conn,
            is_final,
        )?);
    }

    let slice = rank.columns.as_mut_slice();
    slice.sort_by(|colume_a, colume_b| {
        if colume_a.total_accepted != colume_b.total_accepted {
            colume_a
                .total_accepted
                .cmp(&colume_b.total_accepted)
                .reverse()
        } else {
            colume_a.time_cost.cmp(&colume_b.time_cost)
        }
    });

    // assgin rank
    let mut rank_count = 0;
    use crate::schema::region_links as region_links_schema;
    let mut last_total_accepted = region_links_schema::table
        .filter(region_links_schema::region.eq(region.clone()))
        .count()
        .get_result::<i64>(conn)? as i32
        + 1;
    let mut last_time_cost = 0;
    for colume in slice.iter_mut() {
        if colume.is_unrated == Some(true) {
            continue;
        }
        if colume.total_accepted < last_total_accepted {
            rank_count += 1;
        } else if colume.time_cost > last_time_cost {
            rank_count += 1;
        }

        last_total_accepted = colume.total_accepted;
        last_time_cost = colume.time_cost;
        colume.rank = Some(rank_count);
    }
    rank.columns = slice.to_vec();

    ACM_RANK_CACHE.write().unwrap().insert(region, rank);

    Ok(())
}

fn build_acm_rank_column(
    contest: Contest,
    access_control_list_column: AccessControlListColumn,
    conn: &PgConnection,
    is_final: bool,
) -> ServiceResult<ACMRankColumn> {
    let region = access_control_list_column.region;
    let user_id = access_control_list_column.user_id;
    let is_unrated = access_control_list_column.is_unrated;

    use crate::schema::users as users_schema;
    let account = users_schema::table
        .filter(users_schema::id.eq(user_id))
        .select(users_schema::account)
        .first::<String>(conn)?;

    let mut rank_column = ACMRankColumn {
        rank: None,
        user_id,
        account,
        total_accepted: 0,
        time_cost: 0,
        is_unrated,
        problem_block: Vec::new(),
    };

    use crate::schema::region_links as region_links_schema;
    let region_links: Vec<RegionLink> = region_links_schema::table
        .filter(region_links_schema::region.eq(region.clone()))
        .order(region_links_schema::inner_id.asc())
        .load(conn)?;

    for region_link in region_links {
        let mut problem_block = ACMProblemBlock {
            inner_id: region_link.inner_id,
            is_accepted: None,
            is_first_accepted: false,
            is_sealed: false,
            try_times: 0,
            last_submit_time: None,
        };

        use crate::schema::submissions as submissions_schema;
        let submissions: Vec<RawSubmission> = submissions_schema::table
            .filter(submissions_schema::user_id.eq(user_id))
            .filter(submissions_schema::region.eq(region.clone()))
            .filter(submissions_schema::problem_id.eq(region_link.problem_id))
            .filter(submissions_schema::is_accepted.is_not_null())
            .order(submissions_schema::submit_time.asc())
            .load(conn)?;

        for submission in submissions {
            let submit_state = get_contest_state(contest.clone(), submission.submit_time);

            if submit_state == ContestState::Preparing {
                continue;
            } else if submit_state == ContestState::SealedRunning && !is_final {
                problem_block.is_accepted = None;
                problem_block.is_sealed = true;
                problem_block.try_times += 1;
                problem_block.last_submit_time = Some(submission.submit_time);
                if submission.is_accepted.unwrap() {
                    break;
                }
            } else if submit_state == ContestState::Running
                || (submit_state == ContestState::SealedRunning && is_final)
            {
                problem_block.is_accepted = submission.is_accepted;
                problem_block.is_sealed = false;
                problem_block.try_times += 1;
                problem_block.last_submit_time = Some(submission.submit_time);
                if submission.is_accepted.unwrap() {
                    rank_column.total_accepted += 1;
                    rank_column.time_cost += 20 * 60 * (problem_block.try_times as i64 - 1)
                        + submission.submit_time.timestamp()
                        - contest.start_time.timestamp();

                    if submissions_schema::table
                        .filter(submissions_schema::region.eq(region.clone()))
                        .filter(submissions_schema::problem_id.eq(region_link.problem_id))
                        .filter(submissions_schema::is_accepted.eq(Some(true)))
                        .filter(submissions_schema::submit_time.lt(submission.submit_time))
                        .count()
                        .get_result::<i64>(conn)?
                        == 0
                    {
                        problem_block.is_first_accepted = true;
                    }
                    break;
                }
            } else {
                break;
            }
        }

        rank_column.problem_block.push(problem_block);
    }

    Ok(rank_column)
}
