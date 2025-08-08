use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn undefined_variable() {
    // Arrange
    let program = r#"
        print_value(y); 
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::UndefinedVariable)
        .stderr("Undefined variable: y");
}

#[test]
fn unknown_type() {
    // Arrange
    let program = r#"
        let a: unknown = 0; 
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::UnknownType)
        .stderr("'unknown' is not a valid type specifier");
}
