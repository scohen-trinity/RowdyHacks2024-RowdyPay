use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetProfileCommand {
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct GetProfileDBTest {
    pub user_id: i32,
    pub display_name: String,
    pub email: String,
    pub img: Option<String>,
}

#[derive(Deserialize)]
pub struct LeaveGroupCommand {
    pub user_id: i32,
    pub group_id: i32,
}
