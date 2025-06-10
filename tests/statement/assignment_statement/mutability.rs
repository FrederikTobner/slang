use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use crate::ErrorCode;

#[test]
fn mutable_with_type_annotation() {
    let program = r#"
        let mut x: i32 = 42;
        x = 50;
        print_value(x);
    "#;
    execute_program_and_assert(program, "50");
}

#[test]
fn mutable_with_type_inference() {
    let program = r#"
        let mut x = 42;
        x = 50;
        print_value(x);
    "#;
    execute_program_and_assert(program, "50");
}

#[test]
fn multiple_assignments_to_mutable() {
    let program = r#"
        let mut x = 10;
        x = 20;
        x = 30;
        x = x + 5;
        print_value(x);
    "#;
    execute_program_and_assert(program, "35");
}

#[test]
fn mixed_mutable_immutable() {
    let program = r#"
        let x = 10;      // immutable
        let mut y = 20;  // mutable
        y = y + x;       // OK: reading from immutable, writing to mutable
        print_value(y);
    "#;
    execute_program_and_assert(program, "30");
}

#[test]
fn block_scope_mutability() {
    let program = r#"
        let mut x = 10;
        {
            x = 20; // Should work, x is mutable in outer scope
            print_value(x);
        }
    "#;
    execute_program_and_assert(program, "20");
}

#[test]
fn with_immutable_variable() {
    let program = r#"
        let x: i32 = 10;
        x = 20; // This should cause an error
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::AssignmentToImmutableVariable,
        "Cannot assign to immutable variable 'x'",
    );
}

#[test]
fn with_immutable_in_expression() {
    let program = r#"
        let x = 10;
        let y = 20;
        x = y + 5; // Should fail
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::AssignmentToImmutableVariable,
        "Cannot assign to immutable variable 'x'",
    );
}
