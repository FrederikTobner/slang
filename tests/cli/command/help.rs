use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn command() {
    // Arrange
    let mut cmd = Command::cargo_bin("slang").unwrap();

    // Act & Assert
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: slang [COMMAND]"));
}

#[test]
fn is_shown_when_no_subcommand() {
    // Arrange
    let mut cmd = Command::cargo_bin("slang").unwrap();

    // Act & Assert
    cmd.assert()
        .code(2)
        .stderr(predicate::str::contains("Usage: slang [COMMAND]"));
}
