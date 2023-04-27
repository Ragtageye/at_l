use std::time::SystemTime;
use std::io;
use std::thread;
use clap::{Command, Arg};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use indicatif::ProgressStyle;
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

//count_time() and time_writer() are what create new timed entries to timelog
fn count_time() -> TimeResults {
    let timer = SystemTime::now();
    let ending = false;
    let mut this_time = TimeResults {
        hours: 0,
        minutes: 0,
        seconds: 0,
        nondividedtime: 0,
    };
    //todo: replace this with the elapsed function of indicatif crate
    let pen = indicatif::ProgressBar::new_spinner();
    let style = ProgressStyle::with_template(
                    "Press Enter When You're Ready [{elapsed_precise}] {spinner:1.cyan/blue} ",
    ).unwrap();
    pen.set_style(style.clone());
    let _whatever = thread::spawn(move || 
        while !ending {
            pen.inc(1);
            
            //  println!("{} seconds have passed", timer.elapsed().unwrap().as_secs())
    });
    let mut stop = String::new();
    // println!("Press Enter When You're Ready");
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
fn time_writer(timer_results:TimeResults, activity: &str) -> Result<()> {
    let current_count = ActivityData {
        activity_name: activity.to_string(),
        activity_date: chrono::offset::Local::now().to_string(),
        time_achieved: timer_results,
    };
    let mut p = file_reader();
    p[activity].as_array_mut().unwrap().push(serde_json::json!(current_count));
    file_write(p);
    Ok(())

}

// file_check() creates time_log.json if it isnt there, file_reader() pulls time_log.json data into scope, file_write() takes in altered json data and outputs it to time_log.json
fn file_check() -> File {
    let output = match File::open("/mnt/c/Users/Alex Christie/Documents/GitHub/at_l/time_log.json") {
        Ok(_) => File::open("/mnt/c/Users/Alex Christie/Documents/GitHub/at_l/time_log.json"),
        Err(_) => {let file = File::create("/mnt/c/Users/Alex Christie/Documents/GitHub/at_l/time_log.json");
                            file.as_ref().expect("File Not Found").write(b"{\n}").unwrap();
                   file},
    };
    output.unwrap()
}
fn file_reader()  -> serde_json::Value {
    file_check();
    let text = std::fs::read_to_string("/mnt/c/Users/Alex Christie/Documents/GitHub/at_l/time_log.json").unwrap();
    let p: serde_json::Value = serde_json::from_str(&text).unwrap();
    p
}
fn file_write(input: serde_json::Value) {
    let path = std::path::Path::new("./time_log.json");
    let out = serde_json::to_string_pretty(&input);
    let mut seq = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .append(false)
                        .create(true)
                        .truncate(true)
                        .open(path)
                        .expect("Unable to open file");
    seq.write_all(out.expect("File Not Found").as_bytes()).expect("Unable to write data");
}

// add_activity() creates a new category, remove_activity() removes an activity(), list_activities() shows all categories, display_activity_times() does what it says
fn add_activity(new_activity: String) {
    let mut p = file_reader();
    p.as_object_mut().unwrap().insert(new_activity.clone(), serde_json::json!([]));
    file_write(p);
    println!("[{:#?}] added, Here is the full list of activities", &new_activity);
    list_activities();
}
fn remove_activity(input: String) {
    let mut p = file_reader();
    p.as_object_mut().unwrap().remove(&input);
    file_write(p);
}
fn list_activities() {
    let p = file_reader();
    let mut _counter: i32 = 1;
    for i in p.as_object().unwrap().keys() {
        println!("[{}] {}", _counter, i);
        _counter += 1;
    }
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
            let t_out = p[act_list[j]].as_array()
                                      .unwrap()[k].as_object().unwrap()
                                      .get("time_achieved").unwrap().as_object().unwrap()
                                      .get("nondividedtime").unwrap().as_i64().unwrap();
            total_time += t_out;
        }
        let hou = total_time / 60 / 60;
        let minu = (total_time / 60) % 60;
        let sec = total_time % 60;
        println!("{}:\n  Hours: {}\n  Minutes: {}\n  Seconds: {}", act_list[j], hou, minu, sec)

    }
}

fn _choose_activity() {
    let mut _choice: i32;
    list_activities();
    println!("Please Choose an Activity");
    todo!()

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
            .action(clap::ArgAction::SetTrue)
            .help("displays available activities")
        )
        .arg(
            Arg::new("show_times")
            .short('s')
            .action(clap::ArgAction::SetTrue)
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

        if let Some(d) = matches.get_one::<bool>("display") {
            if *d {
            list_activities();
            };
        };
        if let Some(s) = matches.get_one::<bool>("show_times") {
            if *s {
                display_activity_times();
            };
            
        };
        if let Some(a) = matches.get_one::<String>("add_new") {
            add_activity(a.to_string());
        }
        if let Some(t) = matches.get_one::<String>("time_activity") {
           time_writer(count_time(), t).unwrap();
        }
        if let Some(r) = matches.get_one::<String>("remove") {
            remove_activity(r.to_string());
        }    
}