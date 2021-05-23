pub mod handler;
mod statistics;
mod utils;

use actix::prelude::*;
use crate::database::Pool;

pub struct JudgeActor {
    pub pool: Pool,
}

impl Actor for JudgeActor {
    type Context = SyncContext<Self>;
}

pub struct JudgeActorAddr {
    pub addr: Addr<JudgeActor>,
}

pub(crate) fn start_judge_actor(opt: crate::cli_args::Opt, pool: Pool) -> Addr<JudgeActor> {
    info!(
        "Spawning {} JudgeActor in SyncArbiter",
        opt.judge_actor_count
    );

    SyncArbiter::start(opt.judge_actor_count, move || JudgeActor {
        pool: pool.clone(),
    })
}
