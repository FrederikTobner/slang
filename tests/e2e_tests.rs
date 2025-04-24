use std::process::Command;
use std::fs;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;


#[test]
fn test_compile_and_run() {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("test.sl");
    let bytecode_path = temp_dir.path().join("test.sip");

    // Create a simple source file
    fs::write(&source_path, "let x: i32 = 42;\nprint_value(x);\n").unwrap();

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
        .stdout(predicate::str::contains("42"));
}

#[test]
fn test_execute_source_directly() {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("test.sl");

    // Create a simple source file
    fs::write(&source_path, "let x: i32 = 5;\nlet y: i32 = 7;\nprint_value(x * y);\n").unwrap();

    // Execute the source directly
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd
        .arg("execute")
        .arg(&source_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("35"));
}

#[cfg(feature = "print-ast")]
#[test]
fn test_ast_printing_feature() {
    let mut cmd = Command::cargo_bin("slang").unwrap();
    let assert = cmd
        .arg("repl")
        .write_stdin("let x: i32 = 10;\nexit\n")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("=== AST ==="));
}

#[cfg(feature = "print-byte_code")]
#[test]
fn test_bytecode_printing_feature() {
    let mut cmd = Command::cargo_bin("slang").unwrap();
    let assert = cmd
        .arg("repl")
        .write_stdin("let x: i32 = 10;\nexit\n")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("=== Bytecode ==="));
}

// Test for more complex programs
#[test]
fn test_function_execution() {
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("functions.sl");

    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }

        let result = add(5, 7);
        print_value(result);
    "#;

    fs::write(&source_path, program).unwrap();

    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd
        .arg("execute")
        .arg(&source_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("12"));
}

// Helper function to create test programs and assert their output
fn execute_program_and_assert(program: &str, expected_output: &str) {
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
}

// Test cases for each binary operation type
#[test]
fn test_addition_operator() {
    let program = r#"
        let a: i32 = 15;
        let b: i32 = 27;
        print_value(a + b);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_subtraction_operator() {
    let program = r#"
        let a: i32 = 50;
        let b: i32 = 8;
        print_value(a - b);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_multiplication_operator() {
    let program = r#"
        let a: i32 = 6;
        let b: i32 = 7;
        print_value(a * b);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_division_operator() {
    let program = r#"
        let a: i32 = 126;
        let b: i32 = 3;
        print_value(a / b);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_string_concatenation() {
    let program = r#"
        let hello = "Hello, ";
        let world = "world!";
        print_value(hello + world);
    "#;
    execute_program_and_assert(program, "Hello, world!");
}