use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payment {
    pub pmt_id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub amt: f32,
    pub date_created: i32,
    pub description: String,
}

impl Payment {
    pub fn new(pmt_id: i32, user_id: i32, group_id: i32, amt: f32, description: String) -> Payment {
        Payment {
            pmt_id,
            user_id,
            group_id,
            amt,
            date_created: Utc::now().timestamp() as i32,
            description,
        }
    }
}
