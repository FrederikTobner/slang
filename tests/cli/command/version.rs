use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn command() {
    // Arrange
    let mut cmd = Command::cargo_bin("slang").unwrap();

    // Act & Assert
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"slang \d+\.\d+\.\d+").unwrap());
}
