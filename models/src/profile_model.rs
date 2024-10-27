use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub user_id: i64,
    pub display_name: String,
    pub email: String,
    pub img: String,
    pub groups: Vec<i64>,
    pub payments: Vec<i64>,
    pub date_created: usize,
}

impl Profile {
    pub fn new(display_name: String, email: String, img: String, date_created: usize) -> Profile {
        Profile {
            user_id: 1,
            display_name,
            email,
            img,
            groups: vec![],
            payments: vec![],
            date_created,
        }
    }
}
