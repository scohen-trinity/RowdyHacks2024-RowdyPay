use serde::{Deserialize, Serialize};
use bigdecimal::ToPrimitive;

use crate::balance_db_models::BalanceDB;

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

impl From<BalanceDB> for Balance {
    fn from(value: BalanceDB) -> Self {
        Balance {
            balance_id: value.balance_id,
            user_id: value.user_id,
            group_id: value.group_id,
            amt: value.amt.to_f32().unwrap(),
        }
    }
}
