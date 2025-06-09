use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;
use slang_error::ErrorCode;

/// Helper function to create test programs and assert their output
/// Runs the file directly, compiles it, and runs the compiled bytecode
///
/// ### Arguments
/// * `program` - The source code of the program to be executed
/// * `expected_output` - The expected output of the program
pub fn execute_program_and_assert(program: &str, expected_output: &str) {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("test_program.sl");
    let bytecode_path = temp_dir.path().join("test_program.sip");

    fs::write(&source_path, program).unwrap();

    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("execute")
        .arg(&source_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected_output));

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

/// Helper function to test for error cases, checking stderr
///
/// ### Arguments
/// * `program` - The source code of the program to be executed
/// * `expected_error_code` - The expected error code (from ErrorCode enum)
/// * `expected_error` - The expected error message
pub fn execute_program_expect_error(
    program: &str,
    expected_error_code: ErrorCode,
    expected_error: &str,
) {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("test_program.sl");

    fs::write(&source_path, program).unwrap();

    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("execute")
        .arg(&source_path)
        .assert()
        .stderr(predicate::str::contains(expected_error_code.to_string()))
        .stderr(predicate::str::contains(expected_error));
}
