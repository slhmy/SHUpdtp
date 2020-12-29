use crate::schema::*;
use shrinkwraprs::Shrinkwrap;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct User {
    pub id: i32,
    #[graphql(skip)]
    pub salt: Option<String>,
    #[graphql(skip)]
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
            role: user.role
        }
    }
}

#[derive(AsChangeset)]
#[table_name="users"]
pub struct UserForm {
    pub salt: Option<String>,
    pub hash: Option<Vec<u8>>,
    pub account: Option<String>,
    pub mobile: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct SlimUser {
    pub id: i32,
    pub role: String,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct LoggedUser(pub Option<SlimUser>);

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            role: user.role
        }
    }
}

impl From<SlimUser> for LoggedUser {
    fn from(slim_user: SlimUser) -> Self {
        LoggedUser(Some(slim_user))
    }
}