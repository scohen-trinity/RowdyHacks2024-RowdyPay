use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize,Debug, Serialize)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: i32,
}

impl Date {
    pub fn new(day: u8, month: u8, year: i32) -> Date {
        Date { day, month, year, }
    }
}
