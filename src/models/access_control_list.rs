use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "access_control_list"]
pub struct AcessControlList {
    pub region: String,
    pub user_id: i32,
}
