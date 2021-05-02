use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "region_access_settings"]
pub struct RegionAccessSettings {
    pub region: String,
    pub salt: Option<String>,
    pub hash: Option<Vec<u8>>,
}
