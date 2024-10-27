use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group {
    pub group_id: i64,
    pub group_name: String,
    pub users: Vec<i64>,
    pub img: String,
}

impl Group {
    pub fn new(group_name: String, users: Vec<i64>, img: String) -> Group {
        Group {
            group_id: 1,
            group_name,
            users,
            img,
        }
    }
}
