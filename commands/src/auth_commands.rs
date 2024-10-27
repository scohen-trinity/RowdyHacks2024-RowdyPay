use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserCommand {
    pub display_name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct AuthUserCommand {
    pub display_name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UserDB {
    pub user_id: i32,
    pub display_name: String,
    pub email: String,
    pub img: Option<String>,
    pub date_created: i32,
}
