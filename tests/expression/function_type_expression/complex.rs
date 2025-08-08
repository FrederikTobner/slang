use crate::test_utils::ProgramAssertion;

#[test]
fn nested_function_calls() {
    // Arrange
    let program = r#"
        fn inner_function(x: i32) -> i32 {
            return x * 2;
        }
        
        fn outer_function(y: i32) -> i32 {
            return inner_function(y) + 1;
        }
        
        let result = outer_function(20);
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("41");
}

#[test]
fn returning_function_call() {
    // Arrange
    let program = r#"
        fn factory() -> i32 {
            return 42;
        }
        
        fn get_factory_result() -> i32 {
            return factory();
        }
        
        let result = get_factory_result();
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}
