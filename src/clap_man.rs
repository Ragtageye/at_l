/*
okay, let's think about what we want
add a main activity should look like this
at_l --add_activity Writing, maybe we can shorten that to -a?
add a sub activity, which should take two strings
at_l --add_sub_activity Writing BermudaW, shorten to -s
add an entry for a specific activity
at_l --count_activity_time BermudaW, shorten to -c
print out the activities
at_l --get_activities, shorten to -g
print out the sub activities
at_l --get_sub_activities Writing, shorten to -b
print out ALL activities,
at_l --get_all_activities, shorten to -G
print out activity time from main table
at_l --get_time Writing, shorten to -t
print out a specific number of recent entries
at_l --get_sub_time Writing 5, shorten to -T, (this gets the 5 most recent entries)
start gui when I get that done
at_l --gui, shorten to -E
 */

pub(crate) mod clap_man {
    use clap::{Arg, arg, ArgMatches, command};
    use crate::db_manager::db_man as db;
    use text_io::read;
    use crate::activity_data::ActivityData;
    use crate::time_results::TimeResults;

    pub fn run_args(args: ArgMatches) {
        if let Some(a) = args.get_one::<String>("add_activity") {
            db::add_primary_activity(a);
        }
        if let Some(s) = args.get_one::<String>("add_sub_activity") {
            println!("Please type in the name of your sub-activity");
            let test = read!();
            db::add_sub_activity(s, test);
        }
        if let Some(c) = args.get_one::<String>("count_activity_time") {
            let time = TimeResults::new().count_time();
            let activity = ActivityData::new(c, time);
            db::add_entry(activity);
        }
        if let Some(g) = args.get_one::<bool>("get_activities") {
            if *g {
                let flushed: Vec<String> = db::return_main_tables();
                for (x, y) in flushed.into_iter().enumerate() {
                    println!("{}: {} - {} seconds", (x + 1), y, db::return_main_times(&y));
                }
            }
        }
        if let Some(b) = args.get_one::<String>("get_sub_activities") {
            let flushed: Vec<String> = db::return_sub_tables(b);
            for (x, y) in flushed.into_iter().enumerate() {
                println!("{}: {} - {} seconds", (x + 1), y, db::return_table_time_total(&y));
            }
        }
        if let Some(G) = args.get_one::<bool>("get_all_activities") {
            if *G {
                let flushed: Vec<String> = db::return_tables();
                for (x, y) in flushed.into_iter().enumerate() {
                    println!("{}: {} - {} seconds", (x + 1), y, db::return_table_time_total(&y));
                }
            };
        }
        if let Some(t) = args.get_one::<String>("get_time") {
            let print_time: u64 = db::return_table_time_total(t);
            println!("Hours: {}, Minutes: {}, Seconds: {}, where the total time (in seconds) is: {}", print_time / 60 / 60, print_time / 60, print_time & 60, print_time);
        }
        if let Some(T) = args.get_one::<String>("get_time_entries") {
            println!("How many entries do you want to read?");
            let entriesNum: u64 = read!();
            db::print_table_rows(T, entriesNum);
        }
        if let Some(E) = args.get_one::<bool>("gui") {
            if *E {println!("GUI !!!!!")};
        }
    }
    pub fn return_arg_array() -> ArgMatches {
        let add_activity: Arg = arg!(-a --add_activity <Main_Activity> "Adds a primary activity (Something basic like chores, or studying");
        let add_sub_activity: Arg = arg!(-s --add_sub_activity <Main_Activity> "Adds a sub activity (E.G. Add Statistics to Studying");
        let count_activity_time: Arg = arg!(-c --count_activity_time <Activity> "Starts a timer for a specified activity");
        let get_activities: Arg = arg!(-g --get_activities "Lists all main activities");
        let get_sub_activities: Arg = arg!(-b --get_sub_activities <Main_Activity> "Lists all sub activities for an activity");
        let get_all_activities: Arg = arg!(-G --get_all_activities "Lists all activities and sub activities");
        let get_time: Arg = arg!(-t --get_time <Activity> "Lists the total time for an activity and all sub activities");
        let get_time_entries: Arg = arg!(-T --get_time_entries <Activity> "Lists the 5 most recent time entries for an activity");
        let gui: Arg = arg!(-E --gui "Runs the gui version of at_l");

        command!().args([
            add_activity,
            add_sub_activity,
            count_activity_time,
            get_activities,
            get_sub_activities,
            get_all_activities,
            get_time,
            get_time_entries,
            gui
        ]).get_matches()
    }
}

