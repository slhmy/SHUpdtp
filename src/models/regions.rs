use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "regions"]
pub struct Region {
    pub name: String,
    pub self_type: String,
    pub title: String,
    pub has_access_policy: bool,
}
