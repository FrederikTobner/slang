use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn test_variable_scoping_in_blocks() {
    // Arrange
    let program = r#"
        let x = 10;
        {
            let x = 20;
            print_value(x);
        }
        print_value(x);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("20\n10");
}

#[test]
fn test_variable_scoping_in_functions() {
    // Arrange
    let program = r#"
        let global_var = 100;
        
        fn test_function() {
            let local_var = 200;
            print_value(global_var);
            print_value(local_var);
        }
        
        test_function();
        print_value(global_var);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("100\n200\n100");
}

#[test]
fn test_variable_shadowing() {
    // Arrange
    let program = r#"
        let value = 1;
        print_value(value);
        
        {
            let value = 3;
            print_value(value);
        }
        
        print_value(value);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("1\n3\n1");
}

#[test]
fn test_variable_out_of_scope_error() {
    // Arrange
    let program = r#"
        {
            let local_var = 42;
        }
        print_value(local_var); // Should be out of scope
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::UndefinedVariable)
        .stderr("Undefined variable");
}
