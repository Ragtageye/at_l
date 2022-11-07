use std::time::SystemTime;
//use std::thread::sleep;
use std::io;
use std::thread;
use clap::{Command, Arg};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use chrono;
use serde_json::{Number, Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;
//mod tui;
//use crate::tui::draw_terminal;

#[derive(Serialize, Deserialize, Debug)]
struct TimeResults {
    hours: u64,
    minutes: u64,
    seconds: u64,
    nondividedtime: u64,
}
#[derive(Serialize, Deserialize, Debug)]
struct ActivityData {
    activity_name: String,
    activity_date: String,
    time_achieved: TimeResults,
}

fn count_time() -> TimeResults {
    let timer = SystemTime::now();
    let ending = false;
    let mut this_time = TimeResults {
        hours: 0,
        minutes: 0,
        seconds: 0,
        nondividedtime: 0,
    };

    let _whatever = thread::spawn(move || 
        while ending == false {
             println!("{} seconds have passed", timer.elapsed().unwrap().as_secs())
    });

    let mut stop = String::new();
    println!("Press Enter When You're Ready");
    io::stdin()
        .read_line(&mut stop)
        .expect("You Idiot! That's Not an Acceptable Input! And You Know It!");

    match timer.elapsed() {
        Ok(elapsed) => {
            let hou = elapsed.as_secs() / 60 / 60;
            let minu = (elapsed.as_secs() / 60) % 60;
            let sec = elapsed.as_secs() % 60;
            this_time = TimeResults {
                hours: hou, 
                minutes: minu,
                seconds: sec, 
                nondividedtime: elapsed.as_secs(),
            };
            println!("{} hours have passed, {} minutes have passed, {} seconds have passed, and total undivided time in seconds: {}", this_time.hours, this_time.minutes, this_time.seconds, this_time.nondividedtime);
            
            
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    } 
    this_time
}

fn file_check() -> () {
    match File::open("time_log.json") {
        Ok(_) => File::open("time_log.json"),
        Err(_) => {let file = File::create("time_log.json");
                            file.as_ref().expect("File Not Found").write(b"{\n}");
                   file},
    };
}

fn file_writer(timer_results:TimeResults, activity: &str) -> Result<()> {
    let current_count = ActivityData {
        activity_name: activity.to_string(),
        activity_date: chrono::offset::Local::now().to_string(),
        time_achieved: timer_results,
    };
    let mut p = file_reader();
    p[activity].as_array_mut().unwrap().push(serde_json::json!(current_count));
    let out = serde_json::to_string_pretty(&p);
    let mut seq = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .append(false)
                        .create(true)
                        .open("time_log.json")
                        .expect("Unable to open file");
    seq.write_all(out.expect("File Not Found").as_bytes()).expect("Unable to write data");

    Ok(())

}

fn file_reader()  -> serde_json::Value {
    file_check();
    let text = std::fs::read_to_string("time_log.json").unwrap();
    let p: serde_json::Value = serde_json::from_str(&text).unwrap();
    p
}

fn list_activities() {
    let p = file_reader();
    let mut counter: i32 = 1;
    for i in p.as_object().unwrap().keys() {
        println!("[{}] {}", counter, i);
        counter += 1;
    }
}

fn add_activity(new_activity: String) {
    let mut p = file_reader();
    p.as_object_mut().unwrap().insert(new_activity.clone(), serde_json::json!([]));
    let out = serde_json::to_string_pretty(&p);
    let mut seq = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .append(false)
                        .create(true)
                        .open("time_log.json")
                        .expect("Unable to open file");
    seq.write_all(out.expect("File Not Found").as_bytes()).expect("Unable to write data");

    println!("[{:#?}] added, Here is the full list of activities", &new_activity);
    list_activities();
}

fn choose_activity() {
    let mut choice: i32;
    list_activities();
    println!("Please Choose an Activity");
    todo!()

}

fn display_activity_times() {
    let p = file_reader();
    let mut act_list = vec![];
    for i in p.as_object().unwrap().keys() {
        act_list.push(i)
    };
    for j in 0..act_list.len() {
        let mut total_time: i64 = 0;

        for k in 0..p[act_list[j]].as_array().unwrap().len() {
            let t_out = p[act_list[j]].as_array().unwrap()[k].as_object().unwrap().get("time_achieved").unwrap().as_object().unwrap().get("nondividedtime").unwrap().as_i64().unwrap();
            total_time += t_out;
        }
        let hou = total_time / 60 / 60;
        let minu = (total_time / 60) % 60;
        let sec = total_time % 60;
        println!("{}:\n  Hours: {}\n  Minutes: {}\n  Seconds: {}", act_list[j], hou, minu, sec)

    }
}

fn remove_activity(input: String) {
    let mut p = file_reader();
    p.as_object_mut().unwrap().remove(&input);
    let out = serde_json::to_string_pretty(&p);
    let mut seq = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .append(false)
                        .create(true)
                        .open("time_log.json")
                        .expect("Unable to open file");
    seq.write_all(out.expect("File Not Found").as_bytes()).expect("Unable to write data");

    println!("{:#?}", p)
}

fn main() {

    let matches = Command::new("at_l")
        .version("0.1.0")
        .author("ME")
        .about("Activity and Time Logger")
        .arg(
            Arg::new("time_activity")
            .short('t')
            .help("records the time for an activity")
        )
        .arg(
            Arg::new("display")
            .short('d')
            .help("displays available activities")
        )
        .arg(
            Arg::new("show_times")
            .short('s')
            .help("shows the time for all activities")
        )
        .arg(
            Arg::new("add_new")
            .short('a')
            .help("records the time for an activity")
        )
        .arg(
            Arg::new("remove")
            .short('r')
            .help("remove the records for an activity")
        )
        .after_help("Tool to record lifetime devotion to an activity \
                     display the help information from --help or -h")
        .get_matches();


        if let Some(d) = matches.get_one::<String>("display") {
            list_activities();
        };

        if let Some(s) = matches.get_one::<String>("show_times") {
            display_activity_times();
        };

        if let Some(a) = matches.get_one::<String>("add_new") {
            add_activity(a.to_string());
        }

        if let Some(t) = matches.get_one::<String>("time_activity") {
           file_writer(count_time(), t);
        }

        if let Some(r) = matches.get_one::<String>("remove") {
            remove_activity(r.to_string());
        }
        // add_activity("talking".to_string());
        // file_writer(count_time(), "studying");
        // choose_activity();
        // display_activity_times();
        //file_check();
        //test_write();
        // println!("{:#?}", file_reader())
    
}