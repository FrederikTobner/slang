use crate::test_utils::ProgramAssertion;

#[test]
fn return_integer() {
    // Arrange
    let program = r#"
        fn test_function() -> i32 {
            return 42;
        }
        
        let result = test_function();
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn return_string() {
    // Arrange
    let program = r#"
        fn test_function() -> string {
            return "hello";
        }
        
        let result = test_function();
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("hello");
}

#[test]
fn return_boolean() {
    // Arrange
    let program = r#"
        fn test_function() -> bool {
            return true;
        }
        
        let result = test_function();
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("true");
}

#[test]
fn return_float() {
    // Arrange
    let program = r#"
        fn test_function() -> f64 {
            return 3.14;
        }
        
        let result = test_function();
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("3.14");
}

#[test]
fn coerce_integer_return() {
    // Arrange
    let program = r#"
        fn test_function() -> i32 {
            return 42 + 123;
        }
        
        let result: i32 = test_function();
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("165");
}

#[test]
fn coerce_float_return() {
    // Arrange
    let program = r#"
        fn test_function() -> f64 {
            return 3.14 + 2.86;
        }
        
        let result: f64 = test_function();
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("6");
}
