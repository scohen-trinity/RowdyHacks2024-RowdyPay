use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: i32,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub img: String,
    pub groups: Vec<i32>,
    pub payments: Vec<i32>,
    pub date_created: i32,
}

impl User {
    pub fn new(user_id: i32, display_name: String, email: String, password: String, img: String, date_created: i32) -> User {
        User {
            user_id: user_id,
            display_name,
            email,
            password,
            img,
            groups: vec![],
            payments: vec![],
            date_created,
        }
    }
}
