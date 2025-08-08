use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn type_mismatch() {
    // Arrange
    let program = r#"
        fn add(x: i32, y: i32) -> i32 {
            return x + y;
        }
        
        let func_var: fn(string) -> i32 = add; // Type mismatch
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch");
}

#[test]
fn parameter_count_mismatch() {
    // Arrange
    let program = r#"
        fn single_param(x: i32) -> i32 {
            return x;
        }
        
        let func_var: fn(i32, i32) -> i32 = single_param; // Parameter count mismatch
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch");
}

#[test]
fn return_mismatch() {
    // Arrange
    let program = r#"
        fn returns_string() -> string {
            return "hello";
        }
        
        let func_var: fn() -> i32 = returns_string; // Return type mismatch
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch");
}
