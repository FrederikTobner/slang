use crate::ErrorCode;
use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};

#[test]
fn basic() {
    let program = r#"
        print_value("hello");
    "#;
    execute_program_and_assert(program, "hello");
}

#[test]
fn empty() {
    let program = r#"
        print_value("");
    "#;
    execute_program_and_assert(program, "");
}

#[test]
fn with_spaces() {
    let program = r#"
        print_value("hello world");
    "#;
    execute_program_and_assert(program, "hello world");
}

#[test]
fn with_escape_sequences() {
    let program = r#"
        print_value("hello\\nworld");
    "#;
    execute_program_and_assert(program, "hello\\\\nworld");
}

#[test]
fn unterminated() {
    let program = r#"
        print_value("unterminated string
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedClosingQuote,
        "Expected closing quote",
    );
}
