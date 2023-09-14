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
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    let args = &["Hello", "there"];
    run(args, outfile)
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
   let args = &["-n", "Hello", "there"];
   let expected_content_path = "tests/expected/hello2.n.txt";
   run(args, expected_content_path)
}

fn run(args: &[&str], expected_file_path: &str) -> TestResult {
    let expected_content = fs::read_to_string(expected_file_path)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected_content);

    Ok(())
}
