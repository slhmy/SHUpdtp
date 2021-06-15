use server_core::database::*;
use server_core::errors::ServiceResult;
use server_core::utils::encryption;
use shupdtp_db::user;
use shupdtp_db::user::models::*;

fn get_conn() -> ServiceResult<PooledConnection> {
    dotenv::dotenv().ok();
    let database_url: String = dotenv::var("DATABASE_URL").unwrap();
    let pool = server_core::database::pool::establish_connection_with_count(
        &database_url, 1
    );
    Ok(db_connection(&pool)?)
}

#[test]
fn single_operations() {
    let conn = get_conn().unwrap();

    let insertable_user = InsertableUser {
        salt: None,
        hash: None,
        account: "somebody".to_string(),
        mobile: None,
        role: "super".to_string(),
    };
    user::operations::insert(&conn, &insertable_user).unwrap();
    let user_form = UserForm {
        salt: None,
        hash: None,
        account: None,
        mobile: None,
        role: Some("student".to_string()),
    };
    user::operations::update_by_account(&conn, "somebody".to_string(), user_form).unwrap();
    let user = user::operations::get_by_account(&conn, "somebody".to_string()).unwrap();
    println!("{:?}", user);
    user::operations::delete_by_id(&conn, user.id).unwrap();
}