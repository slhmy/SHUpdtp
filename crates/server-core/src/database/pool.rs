use super::{ConnectionManager, Pool, PoolError};

pub fn init_pool(database_url: &str, max_size: u32) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::new(database_url);
    Pool::builder().max_size(max_size).build(manager)
}

pub fn establish_connection(opt: crate::cli_args::Opt) -> Pool {
    let max_size = opt.judge_actor_count as u32 + 10;
    log::info!("Initing Database Pool with max_size={}", max_size);
    init_pool(&opt.database_url, max_size).expect("Failed to create pool")
}
