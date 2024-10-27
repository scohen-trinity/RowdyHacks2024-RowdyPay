use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Balance {
    pub balance_id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub amt: f32,
}

impl Balance {
    pub fn new(user_id: i32, group_id: i32, amt: f32) -> Balance {
        Balance {
            balance_id: 1,
            user_id,
            group_id,
            amt,
        }
    }
}
