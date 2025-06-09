use crate::test_utils::execute_program_expect_error;
use crate::ErrorCode;

#[test]
fn missing_semicolon() {
    let program = r#"
        let a = 42
        print_value(a);
    "#;
    execute_program_expect_error(program, ErrorCode::ExpectedSemicolon, "Expected \';\' after let statement");
}

#[test]
fn mismatched_brackets() {
    let program = r#"
        fn test() {
    "#;
    execute_program_expect_error(program, ErrorCode::ExpectedClosingBrace, "Expected \'}\' after block");
}

#[test]
fn mismatch_quotes() {
    let program = r#"
        let message = "Hello, world!;
        print_value(message);
    "#;
    execute_program_expect_error(program, ErrorCode::ExpectedClosingQuote, "Expected closing quote for string");
}

#[test]
fn mismatched_parentheses() {
    let program = r#"
        let a = 42;
        print_value(a;
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedClosingParen,
        "Expected \')\' after function arguments",
    );
}

#[test]
fn invalid_assignment() {
    let program = r#"
        let a = 42;
        42 = a;
    "#;
    execute_program_expect_error(program, ErrorCode::ExpectedSemicolon, "Expected \';\' after expression");
}

#[test]
fn invalid_variable_declaration() {
    let program = r#"
        let 123abc = 42;
        print_value(123abc);
    "#;
    execute_program_expect_error(program, ErrorCode::ExpectedIdentifier, "Expected identifier after \'let\'");
}

#[test]
fn invalid_function_declaration() {
    let program = r#"
        fn 123invalid() {
            print_value(42);
        }
    "#;
    execute_program_expect_error(program, ErrorCode::ExpectedIdentifier, "Expected function name");
}

#[test]
fn redefined_variable() {
    let program = r#"
        let a = 42;
        let a = 43;
        print_value(a);
    "#;
    execute_program_expect_error(program, ErrorCode::VariableRedefinition, "Variable \'a\' already defined");
}

