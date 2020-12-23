use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
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
    pub role: String,
} 

impl From<User> for OutUser {
    fn from(user: User) -> OutUser {
        OutUser {
            id: user.id,
            account: user.account,
            role: user.role
        }
    }
}