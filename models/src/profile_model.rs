use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub id: u64,
    pub display_name: String,
    pub email: String,
    pub image: String,
    pub groups: Vec<u64>,
    pub payments: Vec<u64>,
    pub date_created: usize,
}

impl Profile {
    pub fn new(display_name: String, email: String, image: String, date_created: usize) -> Profile {
        Profile {
            id: 1,
            display_name,
            email,
            image,
            groups: vec![],
            payments: vec![],
            date_created,
        }
    }
}
