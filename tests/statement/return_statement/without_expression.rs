use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn returns_unit_implicitly() {
    // Arrange
    let program = r#"
        fn test_function() -> () {
            return;
        }
        
        test_function();
        print_value("completed");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("completed");
}

#[test]
fn without_expression_in_non_unit_function_error() {
    // Arrange
    let program = r#"
        fn test_function() -> i32 {
            return;
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::MissingReturnValue)
        .stderr("Type mismatch");
}
