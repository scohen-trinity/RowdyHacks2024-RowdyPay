use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub users: Vec<u64>,
    pub image: String,
}

impl Group {
    pub fn new(name: String, users: Vec<u64>, image: String) -> Group {
        Group {
            id: 1,
            name,
            users,
            image,
        }
    }
}
