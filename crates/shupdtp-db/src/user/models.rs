use crate::schema::*;
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub salt: Option<String>,
    pub hash: Option<Vec<u8>>,
    pub account: String,
    pub mobile: Option<String>,
    pub role: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub salt: Option<String>,
    pub hash: Option<Vec<u8>>,
    pub account: String,
    pub mobile: Option<String>,
    pub role: String,
}

#[derive(Serialize)]
pub struct OutUser {
    pub id: i32,
    pub account: String,
    pub mobile: Option<String>,
    pub role: String,
}

impl From<User> for OutUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            account: user.account,
            mobile: user.mobile,
            role: user.role,
        }
    }
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UserForm {
    pub salt: Option<String>,
    pub hash: Option<Vec<u8>>,
    pub account: Option<String>,
    pub mobile: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: i32,
    pub role: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            role: user.role,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub sup: Option<Vec<String>>,
    pub admin: Option<Vec<String>>,
    pub teacher: Option<Vec<String>>,
    pub student: Option<Vec<String>>,
    pub net_friend: Option<Vec<String>>,
    pub others: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubmissionCount {
    pub total_submit_times: i32,
    pub total_accept_times: i32,
    pub navie_submit_times: i32,
    pub navie_accept_times: i32,
    pub easy_submit_times: i32,
    pub easy_accept_times: i32,
    pub middle_submit_times: i32,
    pub middle_accept_times: i32,
    pub hard_submit_times: i32,
    pub hard_accept_times: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubmissionTime {
    pub date: NaiveDate,
    pub count: i32,
}
