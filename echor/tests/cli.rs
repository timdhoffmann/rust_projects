use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// An 'alias type' used to return the unit type, or an error,
// in failure cases, e.g. when a file to read doesn't exist.
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_without_arguments() -> TestResult {
    // Uses '?' instead of 'Result::unwrap' to unpack an 'Ok' value or propagate
    // an 'Err'.
    let mut cmd = Command::cargo_bin("echor")?;
    let message = "required arguments were not provided".to_string();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(message));

    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.arg("Hello there").assert().success().stdout(expected);

    Ok(())
}

#[test]
fn hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
