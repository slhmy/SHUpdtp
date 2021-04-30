use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "region_links"]
pub struct RegionLink {
    pub region: String,
    pub inner_id: i32,
    pub problem_id: i32,
    pub score: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRegionLinksResult {
    pub problem_id: i32,
    pub inner_id: Option<i32>,
    pub is_success: bool,
}