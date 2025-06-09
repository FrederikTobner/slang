use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use crate::ErrorCode;

#[test]
fn basic_if_statement_true() {
    let program = r#"
        let x: i32 = 5;
        if x > 3 {
            print_value("condition is true");
        }
    "#;
    execute_program_and_assert(program, "condition is true");
}

#[test]
fn basic_if_statement_false() {
    let program = r#"
        let x: i32 = 2;
        if x > 3 {
            print_value("condition is true");
        }
        print_value("after if");
    "#;
    execute_program_and_assert(program, "after if");
}

#[test]
fn if_else_statement_true() {
    let program = r#"
        let x: i32 = 5;
        if x > 3 {
            print_value("true branch");
        } else {
            print_value("false branch");
        }
    "#;
    execute_program_and_assert(program, "true branch");
}

#[test]
fn if_else_statement_false() {
    let program = r#"
        let x: i32 = 2;
        if x > 3 {
            print_value("true branch");
        } else {
            print_value("false branch");
        }
    "#;
    execute_program_and_assert(program, "false branch");
}

#[test]
fn if_statement_multiple_statements() {
    let program = r#"
        let x: i32 = 5;
        if x > 3 {
            print_value("first");
            print_value("second");
        }
    "#;
    execute_program_and_assert(program, "first");
}

#[test]
fn if_else_multiple_statements() {
    let program = r#"
        let x: i32 = 2;
        if x > 3 {
            print_value("true1");
            print_value("true2");
        } else {
            print_value("false1");
            print_value("false2");
        }
    "#;
    execute_program_and_assert(program, "false1");
}

#[test]
fn nested_if_statements() {
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 10;
        if x > 3 {
            if y > 8 {
                print_value("nested true");
            } else {
                print_value("nested false");
            }
        }
    "#;
    execute_program_and_assert(program, "nested true");
}

#[test]
fn if_with_non_boolean_condition() {
    let program = r#"
        let x: i32 = 5;
        if x {
            print_value("should not work");
        }
    "#;
    execute_program_expect_error(program, ErrorCode::TypeMismatch, "Type mismatch");
}

#[test]
fn if_with_string_condition() {
    let program = r#"
        let x: string = "hello";
        if x {
            print_value("should not work");
        }
    "#;
    execute_program_expect_error(program, ErrorCode::TypeMismatch, "Type mismatch");
}

#[test]
fn if_statement_with_complex_condition() {
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 3;
        if x > y && x < 10 {
            print_value("complex condition works");
        }
    "#;
    execute_program_and_assert(program, "complex condition works");
}
