use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "access_control_list"]
pub struct AccessControlListColumn {
    pub user_id: i32,
    pub region: String,
    pub is_unrated: Option<bool>,
}
