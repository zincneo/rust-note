use super::*;

mod cli_mock;

#[test]
fn test_1() {
    Model::init();
    Model::deinit();
}

#[test]
fn test_2() {
    Model::init();
    cli_mock::auto_mock();
    Model::deinit();
}
