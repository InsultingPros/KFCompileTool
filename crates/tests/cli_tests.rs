use assert_cmd::cargo::CommandCargoExt as _;
use serial_test::serial;
use std::process::Command;

#[test]
#[serial]
fn empty_args() {
    assert!(
        Command::cargo_bin("kf_compile_tool")
            .expect("no bin found!")
            .status()
            .expect("failed to execute process")
            .success()
    );
}
