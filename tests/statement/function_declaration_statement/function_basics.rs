use crate::test_utils::ProgramAssertion;

#[test]
fn with_multiple_params() {
    // Arrange
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        print_value(add(20, 22));
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn function_with_no_params() {
    // Arrange
    let program = r#"
        fn get_magic_number() -> i32 {
            return 420;
        }
        
        print_value(get_magic_number());
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("420");
}

#[test]
fn empty_return() {
    // Arrange
    let program = r#"
        fn void_function() {
            return ();
        }
        
        void_function();
        print_value(42); // Just to verify program continues
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn with_no_return() {
    // Arrange
    let program = r#"
        fn no_return_function() {
            // No return statement
        }
        
        no_return_function();
        print_value(42); // Just to verify program continues
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn nested_function_calls() {
    // Arrange
    let program = r#"
        fn add(a: i32, b: i32)-> i32 {
            return a + b;
        }
        
        fn multiply(a: i32, b: i32) -> i32 {
            return a * b;
        }
        
        print_value(add(multiply(3, 10), 12));
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn arguments_are_passed_by_value() {
    // Arrange
    let program = r#"
        fn modify_value(x: i32) -> i32 {
            x = x + 10;
            return x;
        }
        
        let mut a : i32 = 5;
        let b = modify_value(a);
        
        print_value(a); // Should print 5, not 15
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("5");
}

#[test]
fn factorial_recursive_function() {
    // Arrange
    let program = r#"
        fn factorial(n: i32) -> i32 {
            print_value(n); // To show recursion depth
            if n <= 1 {
                return 1;
            }
            return n * factorial(n - 1);
        }
        
        print_value(factorial(5));
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("120");
}

#[test]
fn fibonacci_recursive_function() {
    // Arrange
    let program = r#"
        fn fibonacci(n: i32) -> i32 {
            if n <= 1 {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        print_value(fibonacci(10)); // Should print 55
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("55");
}

#[test]
fn unit_function_parameter() {
    // Arrange
    let program = r#"
        fn test_fn(param: ()) -> () {
            return param;
        }
        
        let result = test_fn(());
        print_value(result);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}

#[test]
fn empty_return_statement() {
    // Arrange
    let program = r#"
        fn test_fn() {
            return;
        }
        
        let result = test_fn();
        print_value(result);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}

#[test]
fn returns_unit_explicitly() {
    // Arrange
    let program = r#"
        fn test_fn() -> () {
            return ();
        }
        
        let result = test_fn();
        print_value(result);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}

#[test]
fn returns_unit_implicitly() {
    // Arrange
    let program = r#"
        fn test_fn() {
            let x = 42;
        }
        
        let result = test_fn();
        print_value(result);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}

#[test]
fn print_function() {
    // Arrange
    let program = r#"
        fn test_fn() {
        }
        
        print_value(test_fn);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("<fn test_fn>");
}

#[test]
fn print_native_function() {
    // Arrange
    let program = r#"
        
        print_value(print_value);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("<native fn print_value>");
}
