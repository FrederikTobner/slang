use crate::ErrorCode;
use crate::assertions::ProgramAssertion;

#[test]
fn let_keyword() {
    // Arrange
    let program = r#"
        let let = 42;
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected identifier");
}

#[test]
fn fn_keyword() {
    // Arrange
    let program = r#"
        let fn = 42;
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected identifier");
}

#[test]
fn if_keyword() {
    // Arrange
    let program = r#"
        let if = 42;
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected identifier");
}

#[test]
fn else_keyword() {
    // Arrange
    let program = r#"
        let else = 42;
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected identifier");
}

#[test]
fn return_keyword() {
    // Arrange
    let program = r#"
        let return = 42;
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected identifier");
}

#[test]
fn struct_keyword() {
    // Arrange
    let program = r#"
        let struct = 42;
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected identifier");
}
