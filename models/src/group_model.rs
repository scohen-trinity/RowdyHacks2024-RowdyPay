use serde::{Deserialize, Serialize};

use crate::profile_model::Profile;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub users: Vec<Profile>,
    pub image: String,
}

impl Group {
    pub fn new(name: String, users: Vec<Profile>, image: String) -> Group {
        Group {
            id: 1,
            name,
            users,
            image,
        }
    }
}
