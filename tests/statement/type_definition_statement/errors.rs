use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn missing_name() {
    // Arrange
    let program = r#"
        struct {
            x: i32,
            y: i32,
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected struct name after \'struct\' keyword");
}

#[test]
fn missing_opening_brace() {
    // Arrange
    let program = r#"
        struct Point
            x: i32,
            y: i32
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedOpeningBrace)
        .stderr("Expected '{' after struct name");
}

#[test]
fn missing_closing_brace() {
    // Arrange
    let program = r#"
        struct Point {
            x: i32,
            y: i32
        // Missing closing brace
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedComma)
        .stderr("Expected \',\' after field or \'}\'");
}

#[test]
fn field_missing_type() {
    // Arrange
    let program = r#"
        struct Point {
            x: i32,
            y: 
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected type identifier");
}

#[test]
fn missing_colon() {
    // Arrange
    let program = r#"
        struct Point {
            x: i32
            y: i32
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedComma)
        .stderr("Expected \',\' after field or \'}\'");
}

#[test]
fn duplicate_definition() {
    // Arrange
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

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::SymbolRedefinition)
        .stderr("Type \'Point\' is already defined in the current scope.");
}
