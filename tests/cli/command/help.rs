use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn command() {
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: slang [COMMAND]"));
}

#[test]
fn is_shown_when_no_subcommand() {
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.assert()
        .code(2)
        .stderr(predicate::str::contains("Usage: slang [COMMAND]"));
}

