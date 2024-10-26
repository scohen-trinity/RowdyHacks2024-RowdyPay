use serde::Serialize;

#[derive(Serialize)]
pub struct GetProfileDTO {
    pub id: u64,
    pub display_name: String,
    pub email: String,
    pub image: String,
    pub groups: Vec<u64>,
    pub payments: Vec<u64>,
    pub date_created: usize,
}
