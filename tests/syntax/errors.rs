use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn missing_semicolon() {
    // Arrange
    let program = r#"
        let a = 42
        print_value(a);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedSemicolon)
        .stderr("Expected \';\' after let statement");
}

#[test]
fn mismatched_brackets() {
    // Arrange
    let program = r#"
        fn test() {
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedClosingBrace)
        .stderr("Expected \'}\' after block");
}

#[test]
fn mismatch_quotes() {
    // Arrange
    let program = r#"
        let message = "Hello, world!;
        print_value(message);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedClosingQuote)
        .stderr("Expected closing quote for string");
}

#[test]
fn mismatched_parentheses() {
    // Arrange
    let program = r#"
        let a = 42;
        print_value(a;
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedClosingParen)
        .stderr("Expected \')\' after function arguments");
}

#[test]
fn invalid_assignment() {
    // Arrange
    let program = r#"
        let a = 42;
        42 = a;
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedSemicolon)
        .stderr("Expected \';\' after expression");
}

#[test]
fn invalid_variable_declaration() {
    // Arrange
    let program = r#"
        let 123abc = 42;
        print_value(123abc);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected identifier after \'let\'");
}

#[test]
fn invalid_function_declaration() {
    // Arrange
    let program = r#"
        fn 123invalid() {
            print_value(42);
        }
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected function name");
}

#[test]
fn redefined_variable() {
    // Arrange
    let program = r#"
        let a = 42;
        let a = 43;
        print_value(a);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::VariableRedefinition)
        .stderr("Variable \'a\' already defined");
}
