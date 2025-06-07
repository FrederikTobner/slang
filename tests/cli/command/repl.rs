use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn exit() {
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("repl")
        .write_stdin("exit\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Slang REPL"));
}

#[test]
fn invalid_syntax() {
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("repl")
        .write_stdin("invalid syntax here\nexit\n")
        .assert()
        .success()
        .stderr(predicate::str::contains("error"));
}
