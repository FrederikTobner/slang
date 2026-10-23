use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn compile_and_run() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let source_file = temp_dir.path().join("test.sl");
    let bytecode_file = temp_dir.path().join("test.sip");

    fs::write(&source_file, "print_value(42);").unwrap();

    // Act & Assert
    Command::cargo_bin("slang")
        .unwrap()
        .arg("compile")
        .arg(&source_file)
        .arg("--output")
        .arg(&bytecode_file)
        .assert()
        .success();

    Command::cargo_bin("slang")
        .unwrap()
        .arg("run")
        .arg(&bytecode_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("42"));
}
