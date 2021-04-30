#[derive(Debug, Clone, Serialize)]
pub struct SizedList<T: serde::Serialize> {
    pub total: i64,
    pub list: Vec<T>,
}
