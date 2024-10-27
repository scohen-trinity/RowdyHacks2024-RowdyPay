use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetProfileCommand {
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct GetProfileDB {
    pub user_id: i32,
    pub display_name: String,
    pub email: String,
    pub img: Option<String>,
    pub groups: Option<Vec<i32>>,
    pub payments: Option<Vec<i32>>,
}

#[derive(Deserialize)]
pub struct GetProfileDBTest {
    pub user_id: i32,
    pub display_name: String,
    pub email: String,
    pub img: Option<String>,
}

