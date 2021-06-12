use server_core::database::*;
use server_core::errors::ServiceResult;
use shupdtp_db::user;

fn get_conn() -> ServiceResult<PooledConnection> {
    dotenv::dotenv().ok();
    let database_url: String = dotenv::var("DATABASE_URL").unwrap();
    let pool = server_core::database::pool::establish_connection_with_count(
        &database_url, 1
    );
    Ok(db_connection(&pool)?)
}

#[test]
fn get_by_id() {
    let conn = get_conn().unwrap();
    assert!(user::operations::get_by_id(&conn, 1).is_ok());
}