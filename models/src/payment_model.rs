use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use utils::date_util::Date;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payment {
    pub pmt_id: i64,
    pub user_id: i64,
    pub group_id: i64,
    pub amt: f64,
    pub date_created: Date,
    pub description: String,
}

impl Payment {
    pub fn new(pmt_id: i64, user_id: i64, group_id: i64, amt: f64, description: String) -> Payment {
        let now = Local::now();
        let current_date = Date::new(now.day() as u8, now.month() as u8 ,now.year());
        Payment {
            pmt_id,
            user_id,
            group_id,
            amt,
            date_created: current_date,
            description,
        }
    }
}
