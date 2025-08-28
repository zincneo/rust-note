use super::*;

mod cli_mock;

#[test]
fn test_1() {
    MODEL.init();
    MODEL.deinit();
}

#[test]
fn test_2() {
    MODEL.init();
    cli_mock::auto_mock();
    MODEL.deinit();
}
