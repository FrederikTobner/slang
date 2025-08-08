use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn basic() {
    // Arrange
    let program = r#"
        print_value(undefined_var);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::UndefinedVariable)
        .stderr("Undefined variable: undefined_var");
}

#[test]
fn in_expression() {
    // Arrange
    let program = r#"
        let x = 10;
        let result = x + undefined_var;
        print_value(result);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::UndefinedVariable)
        .stderr("Undefined variable: undefined_var");
}

#[test]
fn in_assignment() {
    // Arrange
    let program = r#"
        let x = undefined_var;
        print_value(x);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::UndefinedVariable)
        .stderr("Undefined variable: undefined_var");
}

#[test]
fn in_function_call() {
    // Arrange
    let program = r#"
        print_value(undefined_var);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::UndefinedVariable)
        .stderr("Undefined variable: undefined_var");
}
