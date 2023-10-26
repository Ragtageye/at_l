use std::io;
use std::time::SystemTime;

pub struct TimeResults {
    hours: u64,
    minutes: u64,
    seconds: u64,
    non_divided_time: u64
}
impl TimeResults {
    pub fn new() -> TimeResults {
        TimeResults {
            hours : 0,
            minutes : 0,
            seconds : 0,
            non_divided_time : 0
        }
    }
    pub fn print_time(&self) {
        println!("Hours: {}, Minutes: {}, Seconds: {}, where the total time (in seconds) is: {}", &self.hours, &self.minutes, &self.seconds, &self.non_divided_time);
    }
    pub fn get_time_tuple(&self) -> String {
        return format!("hours: {}, minutes: {}, seconds: {}, total time(in seconds): {}", self.hours, self.minutes, self.seconds, self.non_divided_time)
    }
    pub fn get_hours(&self) -> u64 {
        self.hours
    }
    pub fn get_minutes(&self) -> u64 {
        self.minutes
    }
    pub fn get_seconds(&self) -> u64 {
        self.seconds
    }
    pub fn get_non_divided_time(&self) -> u64 {
        self.non_divided_time
    }
    pub fn count_time(mut self) -> TimeResults {
        let timer: SystemTime = SystemTime::now();
        println!("Press enter to continue");
        let _= io::stdin().read_line(&mut "".to_string()).expect("Okay I don't know how you messed that up");
        match timer.elapsed() {
            Ok(elapsed) => {
                self.hours = elapsed.as_secs() / 60 / 60;
                self.minutes = elapsed.as_secs() / 60;
                self.seconds = elapsed.as_secs() % 60;
                self.non_divided_time = elapsed.as_secs();
            }
            Err(e) => println!("Error {e:?}")
        }
        println!("Seconds: {}", self.seconds);
        self
    }

}
impl Default for TimeResults {
    fn default() -> Self {
        TimeResults {
            hours : 0,
            minutes : 0,
            seconds : 0,
            non_divided_time : 0,
        }

    }
}