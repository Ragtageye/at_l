pub(crate) mod db_man {
    use rusqlite::{Connection, params, Rows, Statement};
    use crate::activity_data::ActivityData;

    fn instance_conn() -> Connection {
        let connection : Connection = Connection::open("at_l.db").expect("Can't access, failed at let in instance_conn()");
        connection
    }
    fn check_for_base_table() {
        instance_conn().execute("create table if not exists Activities (
                Activity_ID integer primary key Unique,
                Activity_Name text not null Unique,
                Activity_Solution text default 'none',
                Total_Activity_Time text default '0',
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
        let conn : Connection = instance_conn();
        let mut stmt : Statement = conn.prepare("SELECT Total_Activity_Time from Activities WHERE Activity_Name = ?1").unwrap();
        let mut rows : Rows = stmt.query([activity_name]).unwrap();

        let mut names : Vec<u64> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            names.push(row.get(0).unwrap());
        }
        names[0]
    }
    fn update_base_time(activity_name: &String, activity_time: u64) {
        let conn: Connection = instance_conn();
        conn.execute(
            &*format!(
                "update Activities
                set Total_Activity_Time = {}
                where Activity_Name = '{}'",
                activity_time, activity_name), ())
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
    pub fn add_primary_activity(activity_name: String) {
            check_for_base_table();

            instance_conn().execute("insert or ignore into Activities (Activity_Name) values(?1)",
                params![&activity_name])
                .expect("**failed at adding new activity in add_primary_activity**\n");

            let table_state: String = format!["create table if not exists {} (\
                Entry_ID integer primary key,\
                Entry_date text null default 'No Apparent Date',\
                Entry_time integer not null,\
                Entry_base_ID int not null default {})", &activity_name, return_base_id(&activity_name)];

            instance_conn().execute(&*table_state, ())
                .expect("**failed to create corresponding table in add_primary_activity**\n");
    }
    pub fn add_sub_activity(activity_prime: String, activity_sub: String) {
        check_for_base_table();

        let sub_entry_table_state: String = format!("create table if not exists {} (\
            Entry_ID integer primary key,\
            Entry_date text null default 'No Apparent Date',\
            Entry_time integer not null,\
            Entry_base_ID int not null default {})", &activity_sub, return_base_id(&activity_prime));

        instance_conn().execute(&*sub_entry_table_state,()
            ).expect("3. failed to create sub activity table in add_sub_activity");
    }
    pub fn add_entry(data: ActivityData) {
        let add_time: u64 = return_base_time(&data.activity_name) + (data.activity_time.get_non_divided_time());
        let table_name: String = format!(
        "insert into {} (Entry_date, Entry_time) values('{}', {})",
        data.activity_name,
        data.activity_date,
        data.activity_time.get_non_divided_time());
        println!("{}", &table_name);

        instance_conn().execute(&*table_name, ()).expect("1. add entry failure");
        update_base_time(&data.activity_name, add_time);
    }
}