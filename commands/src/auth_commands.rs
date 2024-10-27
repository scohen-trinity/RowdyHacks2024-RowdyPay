use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserCommand {
    pub display_name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct AuthUserCommand {
    pub email: String,
}
