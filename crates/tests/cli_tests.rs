use serial_test::serial;

#[test]
#[serial]
fn temp() {
    assert_eq!(10, 10);
}
