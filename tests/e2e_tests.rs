use std::process::Command;
use std::fs;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_compile_and_run() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("test.sl");
    let bytecode_path = temp_dir.path().join("test.sip");

    fs::write(&source_path, "let x: i32 = 42;\nprint_value(x);\n").unwrap();

    // Act
    let mut compile_cmd = Command::cargo_bin("slang").unwrap();
    compile_cmd
        .arg("compile")
        .arg(&source_path)
        .arg("-o")
        .arg(&bytecode_path)
        .assert()
        .success();

    let mut run_cmd = Command::cargo_bin("slang").unwrap();
    run_cmd
        .arg("run")
        .arg(&bytecode_path)

    // Assert
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

// Helper function to test for error cases, checking stderr
fn execute_program_expect_error(program: &str, expected_error: &str) {
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

#[test]
fn test_negation_operator() {
    let program = r#"
        let a: i32 = 42;
        print_value(-a);
    "#;
    execute_program_and_assert(program, "-42");
}

#[test]
fn test_negation_operator_on_string_error() {
    let program = r#"
        let a: string = "Hello";
        print_value(-a);
    "#;
    
    let temp_dir = tempdir().unwrap();
    let source_path = temp_dir.path().join("test_program.sl");
    fs::write(&source_path, program).unwrap();
    let mut cmd = Command::cargo_bin("slang").unwrap(); 

    cmd
        .arg("execute")
        .arg(&source_path)
        .assert()
        .stderr(predicate::str::contains("Cannot negate non-numeric type"));
}

#[test]
fn test_missing_semicolon() {
    let program = r#"
        let x: i32 = 10
    "#;
    
    execute_program_expect_error(program, "Compilation failed: Expected \';\' after let statement\n");
}

#[test]
fn test_assign_string_literal_to_i32() {
    let program = r#"
        let x: i32 = "not an integer"; 
        print_value(x);
    "#;
    
    execute_program_expect_error(program, "Compilation failed: Type mismatch: variable x is i32 but expression is string\n");
}

#[test]
fn test_function_call_param_order() {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return 2 * a + b;
        }

        let result = add(5, 7);
        print_value(result);
    "#;
    
    execute_program_and_assert(program, "17");
}

#[test]
fn test_function_call_param_type_mismatch() {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }

        let result = add(5, "not an integer");
    "#;
    
    execute_program_expect_error(program, "Compilation failed: Type mismatch: function \'add\' expects argument 2 to be i32, but got string\n");
}

#[test]
fn test_function_call_param_count_mismatch() {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }

        let result = add(5);
    "#;
    
    execute_program_expect_error(program, "Compilation failed: Function \'add\' expects 2 arguments, but got 1\n");
}

#[test]
fn test_division_by_zero() {
    let program = r#"
        let x: i32 = 42;
        print_value(x / 0);
    "#;
    
    execute_program_expect_error(program, "Runtime error: Division by zero\n");
}
