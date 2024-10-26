use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use utils::date_util::Date;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payment {
    pub pmt_id: u64,
    pub user_id: u64,
    pub group_id: u64,
    pub amt: f64,
    pub date_made: Date,
}

impl Payment {
    pub fn new(pmt_id: u64, user_id: u64, group_id: u64, amt: f64) -> Payment {
        let now = Local::now();
        let current_date = Date::new(now.day() as u8, now.month() as u8 ,now.year());
        Payment {
            pmt_id,
            user_id,
            group_id,
            amt,
            date_made: current_date,
        }
    }
}
