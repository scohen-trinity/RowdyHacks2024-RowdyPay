use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Balance {
    pub balance_id: i64,
    pub user_id: i64,
    pub group_id: i64,
    pub amt: f64,
}

impl Balance {
    pub fn new(user_id: i64, group_id: i64, amt: f64) -> Balance {
        Balance {
            balance_id: 1,
            user_id,
            group_id,
            amt,
        }
    }
}
