use serde::Serialize;

#[derive(Serialize)]
pub struct GetProfileDTO {
    pub user_id: i32,
    pub display_name: String,
    pub email: String,
    pub groups: Vec<i32>,
    pub payments: Vec<i32>,
    pub img: String,
}
