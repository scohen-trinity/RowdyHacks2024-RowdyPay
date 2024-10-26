use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Balance {
    pub user_id: u64,
    pub group_id: u64,
    pub amt: f64,
}

impl Balance {
    pub fn new(user_id: u64, group_id: u64, amt: f64) -> Balance {
        Balance {
            user_id,
            group_id,
            amt,
        }
    }
}
