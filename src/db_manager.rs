pub(crate) mod db_man {
    use std::path::{PathBuf};
    use rusqlite::{Connection, params, Rows, Statement};
    use crate::activity_data::ActivityData;
    use directories::UserDirs;
    fn instance_conn() -> Connection {
        let path_source:UserDirs = UserDirs::new().expect("Could not find Docs");
        let mut path = PathBuf::from(path_source.document_dir().unwrap().to_str().unwrap().to_owned() + "/at_l");
        if !path.exists() {
            std::fs::create_dir(&path).expect("No Write Access");
        }
        path.push("at_l.db");
        let connection : Connection = Connection::open(path).expect("Can't access, failed at let in instance_conn()");
        connection
    }
    fn check_for_base_table() {
        instance_conn().execute("create table if not exists Activities (
                Activity_ID integer primary key Unique,
                Activity_Name text not null Unique,
                Activity_Solution text default 'none',
                Total_Activity_Time int default 0,
                Unique(Activity_Name, Activity_Solution)
            )",
        ()
        ).expect("What the hell did you type wrong: check_for_base_table() error");

        instance_conn().execute(
            "insert or ignore into Activities (Activity_Name) values('Default')",
            ()
        ).expect("Failed to insert default activity");

    }
    fn return_base_time(activity_name: &String) -> u64 {
        check_for_base_table();
        let conn : Connection = instance_conn();
        let mut stmt : Statement = conn.prepare("SELECT Total_Activity_Time from Activities WHERE Activity_Name = ?1").expect("rbt failure @ 1");
        let mut rows : Rows = stmt.query([activity_name]).expect("rbt failure @ 2");

        let mut names : Vec<u64> = Vec::new();
        while let Some(row) = rows.next().expect("rbt failure @ 3") {
            names.push(row.get(0).expect("rbt failure @ 4"));
        }
        println!("Here's the base time: {}", names[0]);
        names[0]
    }
    fn update_base_time(activity_name: &String, activity_time: u64) {
        let conn: Connection = instance_conn();
        let table_state: String = format!(
            "update Activities
             set Total_Activity_Time = {}
             where Activity_Name = '{}'",
            activity_time, return_primary_activity(activity_name));
        conn.execute(&table_state, ())
            .expect("panic at update base time");
    }
    fn return_base_id(activity_name: &String) -> u64 {
        let conn : Connection = instance_conn();
        let mut stmt : Statement = conn.prepare("SELECT Activity_ID from Activities WHERE Activity_Name = ?1").unwrap();
        let mut rows : Rows = stmt.query([activity_name]).unwrap();
        
        let mut names : Vec<u64> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            names.push(row.get(0).unwrap());
        }
        names[0]
    }
    fn return_primary_activity(activity_name: &String) -> String {
        let conn : Connection = instance_conn();
        let table_state = format!
        ("SELECT Activity_Name
         FROM {}
         JOIN Activities on {}.Entry_base_ID=Activity_ID",
         &activity_name, &activity_name);
        let mut stmt : Statement = conn.prepare(&table_state).expect("RPA failed @ 1");
        let mut rows : Rows = stmt.query([]).expect("RPA failed @ 2");

        let mut names : Vec<String> = Vec::new();
        while let Some(row) = rows.next().expect("RPA failed @ 3") {
            names.push(row.get(0).expect("RPA failed @ 4"));
        }
        names[0].clone()
    }
    pub fn add_primary_activity(activity_name: &String) {
            check_for_base_table();

            instance_conn().execute("insert or ignore into Activities (Activity_Name) values(?1)",
                params![&activity_name])
                .expect("**failed at adding new activity in add_primary_activity**\n");

            let table_state: String = format!["create table if not exists {} (\
                Entry_ID integer primary key,\
                Entry_date text null default 'No Apparent Date',\
                Entry_time integer not null,\
                Entry_base_ID int not null default {})", &activity_name, return_base_id(activity_name)];

            instance_conn().execute(&table_state, ())
                .expect("**failed to create corresponding table in add_primary_activity**\n");

            let up_state: String = format!["insert into {} (Entry_time) Values ({})", &activity_name, 0];
            instance_conn().execute(&up_state, ()).expect("**final failure in apa");
    }
    pub fn add_sub_activity(activity_prime: &String, activity_sub: String) {
        check_for_base_table();

        let comb_entry_table_state: String = format!("create table if not exists SUB_{} (Entry_Name text not null primary key);",
            &activity_prime);
        instance_conn().execute(&comb_entry_table_state, ()).expect("2. Failed to create sub activity sub_table");

        let entry_instance: String = format!("insert into SUB_{} (Entry_Name) Values('{}');", &activity_prime, &activity_sub);
        instance_conn().execute(&entry_instance, ()).expect("2.5 Failed to add entry");

        let sub_entry_table_state: String = format!("create table if not exists {} (\
            Entry_ID integer primary key,\
            Entry_date text null default 'No Apparent Date',\
            Entry_time integer not null,\
            Entry_base_ID int not null default {})", &activity_sub, return_base_id(activity_prime));

        instance_conn().execute(&sub_entry_table_state,()
            ).expect("3. failed to create sub activity table in add_sub_activity");

        let update_state: String = format!["insert into {} (Entry_time) Values ({})", &activity_sub, 0];
        instance_conn().execute(&update_state, ()).expect("4. failed to create sub activity table");
    }
    pub fn add_entry(data: ActivityData) {
        let add_time: u64 = return_base_time(&return_primary_activity(&data.activity_name)) + (data.activity_time.get_non_divided_time());
        let table_name: String = format!(
        "insert into {} (Entry_date, Entry_time) values('{}', {})",
        data.activity_name,
        data.activity_date,
        data.activity_time.get_non_divided_time());
        println!("{}", &table_name);

        instance_conn().execute(&table_name, ()).expect("1. add entry failure");
        update_base_time(&data.activity_name, add_time);
    }
    // lots of redundant code here, just want to get it working. I can fix it later
    pub fn return_tables() -> Vec<String>{
        let conn : Connection = instance_conn();
        let mut stmt : Statement = conn.prepare("SELECT name from sqlite_master WHERE sql is not null and name is not 'Activities' and name not like 'SUB%'").unwrap();
        let mut rows : Rows = stmt.query([]).unwrap();
        let mut names : Vec<String> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            names.push(row.get(0).unwrap());
        }
        names
    }
    pub fn return_main_tables() -> Vec<String> {
        let conn : Connection = instance_conn();
        let mut stmt : Statement = conn.prepare
        ("SELECT Activity_Name from Activities
              WHERE Activity_Name is not 'Default'").unwrap();

        let mut rows : Rows = stmt.query([]).unwrap();
        let mut names : Vec<String> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            names.push(row.get(0).unwrap());
        }
        names
    }
    pub fn return_sub_tables(prime_activity: &String) -> Vec<String>{
        let conn : Connection = instance_conn();
        let mut stmt : Statement = conn.prepare
        (&format!("SELECT Entry_Name from SUB_{}", prime_activity)).unwrap();

        let mut rows : Rows = stmt.query([]).unwrap();
        let mut names : Vec<String> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            names.push(row.get(0).unwrap());
        }
        names
    }
    pub fn return_table_time_total(activity_name: &String) -> u64 {
        let conn: Connection = instance_conn();
        let mut stmt: Statement = conn.prepare(&format!("SELECT sum(Entry_time) from {};", &activity_name)).expect("1. failed at time grab");

        let mut rows: Rows = stmt.query([]).expect("failed at rows");
        let mut names: Vec<u64> = Vec::new();

        while let Some(row) = rows.next().unwrap() {
            names.push(row.get(0).unwrap());
        }
        names[0]
    }
    pub fn return_main_times(activity_name: &String) -> u64 {
        let conn: Connection = instance_conn();
        let mut stmt: Statement = conn.prepare(&format!("SELECT Total_Activity_Time from Activities WHERE Activity_Name = '{}';", &activity_name)).expect("1. failed at time grab");

        let mut rows: Rows = stmt.query([]).expect("failed at rows");
        let mut names: Vec<u64> = Vec::new();

        while let Some(row) = rows.next().unwrap() {
            names.push(row.get(0).unwrap());
        }
        names[0]
    }
    pub fn print_table_rows(activity_name: &String, idx_count: u64) {
        let conn: Connection = instance_conn();
        let mut stmt: Statement = conn.prepare(&format!("SELECT * from {} WHERE Entry_ID is not 1", activity_name)).expect("print rows failure");

        let mut rows: Rows = stmt.query([]).expect("failed at rows");
        let mut rev_rows: Vec<String> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            let v1 : u64 = row.get_unwrap(0);
            let mut v2 : String = row.get_unwrap(1);
            v2.replace_range(19..36, "");
            let v3 : u64 = row.get_unwrap(2);
            let v4 : u64 = row.get_unwrap(3);
            rev_rows.push(format!("Entry_ID: {}, Entry_date: {}, Entry_time: {}, Entry_base_ID: {}", v1, v2.as_str(), v3, v4));
        }
        for (x, y) in rev_rows.iter().enumerate().rev() {
            println!("{}:  {}", rev_rows.len() -  x, y);
            if x == (rev_rows.len() - idx_count as usize) {
                break;
            }
        }
    }
}