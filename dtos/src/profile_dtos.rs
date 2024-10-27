use serde::Serialize;

#[derive(Serialize)]
pub struct GetProfileDTO {
    pub id: i64,
    pub display_name: String,
    pub email: String,
    pub image: String,
    pub groups: Vec<i64>,
    pub payments: Vec<i64>,
    pub date_created: usize,
}
