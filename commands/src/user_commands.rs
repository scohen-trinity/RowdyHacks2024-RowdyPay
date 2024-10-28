use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetUserCommand {
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct GetUserDBTest {
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
