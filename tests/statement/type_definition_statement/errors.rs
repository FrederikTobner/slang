use crate::ErrorCode;
use crate::test_utils::execute_program_expect_error;

#[test]
fn missing_name() {
    let program = r#"
        struct {
            x: i32,
            y: i32,
        }
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedIdentifier,
        "Expected struct name after \'struct\' keyword",
    );
}

#[test]
fn missing_opening_brace() {
    let program = r#"
        struct Point
            x: i32,
            y: i32
        }
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedOpeningBrace,
        "Expected '{' after struct name",
    );
}

#[test]
fn missing_closing_brace() {
    let program = r#"
        struct Point {
            x: i32,
            y: i32
        // Missing closing brace
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedComma,
        "Expected \',\' after field or \'}\'",
    );
}

#[test]
fn field_missing_type() {
    let program = r#"
        struct Point {
            x: i32,
            y: 
        }
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedIdentifier,
        "Expected type identifier",
    );
}

#[test]
fn missing_colon() {
    let program = r#"
        struct Point {
            x: i32
            y: i32
        }
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedComma,
        "Expected \',\' after field or \'}\'",
    );
}

#[test]
fn duplicate_definition() {
    let program = r#"
        struct Point {
            x: i32,
            y: i32,
        };
        struct Point {
            x: i32,
            y: i32,
        };
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::SymbolRedefinition,
        "Type \'Point\' is already defined in the current scope.",
    );
}
