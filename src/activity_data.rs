use chrono::{DateTime, offset::Local};
use crate::time_results::TimeResults;

pub struct ActivityData {
    pub activity_name: String,
    pub activity_date: DateTime<Local>,
    pub activity_time: TimeResults,
}

impl ActivityData {
    pub fn new(a_name: &String, times: TimeResults) -> ActivityData {
        ActivityData {
            activity_name : a_name.to_string(),
            activity_date : Local::now(),
            activity_time : times,
        }
    }
    pub fn print_activity_data(&self) {
        println!("Activity Name: {}, Activity Date and Time: {}, Activity Time Elapsed/Spend: ({})", self.activity_name, self.activity_date, self.activity_time.get_time_tuple())
    }

}