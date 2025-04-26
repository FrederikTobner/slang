use std::process::Command;
use std::fs;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;

// Helper function to create test programs and assert their output
pub fn execute_program_and_assert(program: &str, expected_output: &str) {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("test_program.sl");

    fs::write(&source_path, program).unwrap();

    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd
        .arg("execute")
        .arg(&source_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected_output));

    compile_and_run(program, expected_output);
}

// Helper function to test for error cases, checking stderr
pub fn execute_program_expect_error(program: &str, expected_error: &str) {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("test_program.sl");

    fs::write(&source_path, program).unwrap();

    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd
        .arg("execute")
        .arg(&source_path)
        .assert()
        .stderr(predicate::str::contains(expected_error));
}

// Helper function to compile and run a program with the two-step process
pub fn compile_and_run(program: &str, expected_output: &str) {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("test_program.sl");
    let bytecode_path = temp_dir.path().join("test_program.sip");

    // Create source file
    fs::write(&source_path, program).unwrap();

    // Compile the source
    let mut compile_cmd = Command::cargo_bin("slang").unwrap();
    compile_cmd
        .arg("compile")
        .arg(&source_path)
        .arg("-o")
        .arg(&bytecode_path)
        .assert()
        .success();

    // Run the compiled bytecode
    let mut run_cmd = Command::cargo_bin("slang").unwrap();
    run_cmd
        .arg("run")
        .arg(&bytecode_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected_output));
}