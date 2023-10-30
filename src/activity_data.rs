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
}