use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn valid_file() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.sl");
    
    fs::write(&input_file, "print_value(42);").unwrap();
    
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("execute")
        .arg(&input_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("42"));
}

#[test]
fn with_runtime_error() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("error.sl");
    
    fs::write(&input_file, "undefined_function();").unwrap();
    
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("execute")
        .arg(&input_file)
        .assert()
        .failure();
}
