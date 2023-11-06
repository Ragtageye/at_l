use crate::clap_man as com;
mod time_results;
mod activity_data;
mod db_manager;
mod clap_man;
mod gui_man;

fn main() {
    com::run_args(com::return_arg_array());
}
