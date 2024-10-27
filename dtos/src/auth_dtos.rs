use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserDB {
    display_name: String,
    email: String,
}