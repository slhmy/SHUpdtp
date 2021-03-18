pub mod handler;
mod utils;

use actix::prelude::*;
use diesel::prelude::*;

pub struct JudgeActor(pub PgConnection);

impl Actor for JudgeActor {
    type Context = SyncContext<Self>;
}

pub struct JudgeActorAddr {
    pub addr: Addr<JudgeActor>,
}

pub(crate) fn start_judge_actor(opt: crate::cli_args::Opt) -> Addr<JudgeActor> {
    let database_url = opt.database_url.clone();

    info!(
        "Spawning {} JudgeActor in SyncArbiter",
        opt.judge_actor_count
    );

    SyncArbiter::start(opt.judge_actor_count, move || {
        JudgeActor(PgConnection::establish(&database_url).unwrap())
    })
}
