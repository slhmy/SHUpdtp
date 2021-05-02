use crate::models::*;
use diesel::prelude::*;
use mongodb::bson::doc;
use mongodb::sync::Database as MongoDB;

pub fn update_column(
    db_connection: &PgConnection,
    mongodb_database: MongoDB,
    submission: submissions::Submission,
) {

}