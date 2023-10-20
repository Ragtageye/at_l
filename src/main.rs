use crate::activity_data::ActivityData;
use crate::time_results::TimeResults;
use crate::db_manager::db_man;
use crate::clap_man::clap_man as com;
mod time_results;
mod activity_data;
mod db_manager;
mod clap_man;

fn main() {
    com::run_args(com::return_arg_array());
}
