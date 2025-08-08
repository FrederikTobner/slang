use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn missing_opening_parenthesize() {
    // Arrange
    let program = r#"
        let my_function2 : fn i32 -> () = my_function;
        "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedOpeningParen)
        .stderr(" Expected \'(\' after \'fn\'");
}

#[test]
fn missing_closing_parentesize() {
    // Arrange
    let program = r#"
        let my_function2 : fn(i32 -> = my_function;
        "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedClosingParen)
        .stderr(" Expected \')\' after function parameters");
}

#[test]
fn missing_type_identifier() {
    // Arrange
    let program = r#"
        let my_function2 : fn(i32) -> = my_function;
        "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr(" Expected type identifier");
}

#[test]
fn expect_arrow() {
    // Arrange
    let program = r#"
        let my_function2 : fn(i32) = my_function;
        "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidSyntax)
        .stderr(" Expected \'->\' after function parameters");
}
