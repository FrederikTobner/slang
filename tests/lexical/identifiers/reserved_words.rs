use crate::ErrorCode;
use crate::test_utils::execute_program_expect_error;

#[test]
fn let_keyword() {
    let program = r#"
        let let = 42;
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedIdentifier,
        "Expected identifier",
    );
}

#[test]
fn fn_keyword() {
    let program = r#"
        let fn = 42;
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedIdentifier,
        "Expected identifier",
    );
}

#[test]
fn if_keyword() {
    let program = r#"
        let if = 42;
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedIdentifier,
        "Expected identifier",
    );
}

#[test]
fn else_keyword() {
    let program = r#"
        let else = 42;
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedIdentifier,
        "Expected identifier",
    );
}

#[test]
fn return_keyword() {
    let program = r#"
        let return = 42;
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedIdentifier,
        "Expected identifier",
    );
}

#[test]
fn struct_keyword() {
    let program = r#"
        let struct = 42;
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedIdentifier,
        "Expected identifier",
    );
}
