use crate::model;
mod cli;

#[test]
fn cli_sense() {
    model::init();
    cli::cli_task();
    model::deinit();
}
