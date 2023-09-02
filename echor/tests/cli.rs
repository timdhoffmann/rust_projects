use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_without_arguments() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    let message = "required arguments were not provided".to_string();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(message));
}

#[test]
fn hello1() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there")
        .assert()
        .success()
        .stdout(expected);
}