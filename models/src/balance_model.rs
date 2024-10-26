use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Balance {
    pub balance_id: u64,
    pub user_id: u64,
    pub group_id: u64,
    pub amt: f64,
}

impl Balance {
    pub fn new(user_id: u64, group_id: u64, amt: f64) -> Balance {
        Balance {
            balance_id: 1,
            user_id,
            group_id,
            amt,
        }
    }
}
