use crate::ErrorCode;
use crate::assertions::ProgramAssertion;

#[test]
fn basic() {
    // Arrange
    let program = r#"
        print_value("hello");
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("hello");
}

#[test]
fn empty() {
    // Arrange
    let program = r#"
        print_value("");
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("");
}

#[test]
fn with_spaces() {
    // Arrange
    let program = r#"
        print_value("hello world");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("hello world");
}

#[test]
fn with_escape_sequences() {
    // Arrange
    let program = r#"
        print_value("hello\\nworld");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("hello\\\\nworld");
}

#[test]
fn unterminated() {
    // Arrange
    let program = r#"
        print_value("unterminated string
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedClosingQuote)
        .stderr("Expected closing quote");
}
