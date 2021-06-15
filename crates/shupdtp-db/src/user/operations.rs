use diesel::prelude::*;
use crate::schema::users as users_schema;
use server_core::{
    errors::ServiceResult,
};
use diesel::pg::PgConnection;
use super::models::*;

pub fn insert(
    conn: &PgConnection,
    insertable_user: &InsertableUser,
) -> ServiceResult<()> {
    diesel::insert_into(users_schema::table)
        .values(insertable_user)
        .execute(conn)?;

    Ok(())
}

pub fn get_by_id(
    conn: &PgConnection,
    id: i32,
) -> ServiceResult<User> {
    Ok(users_schema::table
        .filter(users_schema::id.eq(id))
        .first::<User>(conn)?
    )
}

pub fn get_by_account(
    conn: &PgConnection,
    account: String,
) -> ServiceResult<User> {
    Ok(users_schema::table
        .filter(users_schema::account.eq(account))
        .first::<User>(conn)?
    )
}

pub fn update_by_id(
    conn: &PgConnection,
    id: i32,
    user_form: UserForm,
) -> ServiceResult<()> {
    diesel::update(users_schema::table.filter(users_schema::id.eq(id)))
        .set(user_form)
        .execute(conn)?;
    
    Ok(())
}

pub fn update_by_account(
    conn: &PgConnection,
    account: String,
    user_form: UserForm,
) -> ServiceResult<()> {
    diesel::update(users_schema::table.filter(users_schema::account.eq(account)))
        .set(user_form)
        .execute(conn)?;
    
    Ok(())
}

pub fn delete_by_id(
    conn: &PgConnection,
    id: i32,
) -> ServiceResult<()> {
    diesel::delete(users_schema::table.filter(users_schema::id.eq(id))).execute(conn)?;
    
    Ok(())
}