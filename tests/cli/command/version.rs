use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn command() {
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"slang \d+\.\d+\.\d+").unwrap());
}

