use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub user_id: i32,
    pub display_name: String,
    pub email: String,
    pub img: String,
    pub groups: Vec<i32>,
    pub payments: Vec<i32>,
    pub date_created: i32,
}

impl Profile {
    pub fn new(user_id: i32, display_name: String, email: String, img: String, date_created: i32) -> Profile {
        Profile {
            user_id: user_id,
            display_name,
            email,
            img,
            groups: vec![],
            payments: vec![],
            date_created,
        }
    }
}
