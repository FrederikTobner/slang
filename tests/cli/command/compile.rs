use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn valid_file() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.sl");
    let output_file = temp_dir.path().join("test.sip");
    
    fs::write(&input_file, "print_value(42);").unwrap();
    
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("compile")
        .arg(&input_file)
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Successfully compiled"));
    
    assert!(output_file.exists());
}

#[test]
fn test_compile_with_default_output() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.sl");
    let expected_output = temp_dir.path().join("test.sip");
    
    fs::write(&input_file, "print_value(42);").unwrap();
    
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("compile")
        .arg("test.sl")
        .assert()
        .success();
    
    assert!(expected_output.exists());
}

#[test]
fn nonexistent_file() {
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("compile")
        .arg("nonexistent.sl")
        .assert()
        .failure()
        .code(66); // NoInput exit code
}

#[test]
fn test_compile_invalid_syntax() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("invalid.sl");
    
    fs::write(&input_file, "invalid syntax here").unwrap();
    
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("compile")
        .arg(&input_file)
        .assert()
        .failure()
        .code(70); // Software exit code
}

#[test]
fn permission_denied_error() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.sl");
    let readonly_output = temp_dir.path().join("readonly.sip");
    
    fs::write(&input_file, "print_value(42);").unwrap();
    fs::write(&readonly_output, "").unwrap();
    
    // Make file read-only
    let mut perms = fs::metadata(&readonly_output).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&readonly_output, perms).unwrap();
    
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("compile")
        .arg(&input_file)
        .arg("--output")
        .arg(&readonly_output)
        .assert()
        .failure()
        .code(77); // NoPerm exit code
}