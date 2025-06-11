use crate::ErrorCode;
use crate::test_utils::execute_program_expect_error;

#[test]
fn missing_opening_parenthesize() {
    let program = r#"
        let my_function2 : fn i32 -> () = my_function;
        "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedOpeningParen,
        " Expected \'(\' after \'fn\'",
    );
}

#[test]
fn missing_closing_parentesize() {
    let program = r#"
        let my_function2 : fn(i32 -> = my_function;
        "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedClosingParen,
        " Expected \')\' after function parameters",
    );
}

#[test]
fn missing_type_identifier() {
    let program = r#"
        let my_function2 : fn(i32) -> = my_function;
        "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedIdentifier,
        " Expected type identifier",
    );
}

#[test]
fn expect_arrow() {
    let program = r#"
        let my_function2 : fn(i32) = my_function;
        "#;
    execute_program_expect_error(
        program,
        ErrorCode::InvalidSyntax,
        " Expected \'->\' after function parameters",
    );
}
