use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn with_explicit_function_type_mismatch() {
    // Arrange
    let program = r#"
        fn my_print(value: string) {
            print_value(value);
        }
        
        let my_function : fn(i32) -> () = my_print;
        "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch: variable my_function is fn(i32) -> () but expression is fn(string) -> ()");
}

#[test]
fn with_explicit_function_type() {
    // Arrange
    let program = r#"
         fn my_print(value: string) {
            print_value(value);
        }
        let my_function : fn(string) -> () = my_print;
        my_function("Hello from native function");
        "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("Hello from native function");
}

#[test]
fn assign_native_to_function_with_explicit_function_type() {
    // Arrange
    let program = r#"
        fn my_print(value: string) {
            print_value(value);
        }
        let my_function : fn(string) -> () = my_print;
        my_function("Hello from native function");
        "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("Hello from native function");
}

#[test]
fn with_explicit_unit_return_type() {
    // Arrange
    let program = r#"
        fn return_unit() -> () {
            return ();
        }
        
        let result = return_unit();
        print_value(result); // Should print nothing or "()" depending on implementation
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}

#[test]
fn with_different_return_types() {
    // Arrange
    let program = r#"
        fn get_string() -> string {
            return "Hello world";
        }
        
        fn get_int() -> i32 {
            return 42;
        }
        
        fn get_float() -> f64 {
            return 42.5;
        }
        
        print_value(get_string());
        print_value(get_int());
        print_value(get_float());
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("Hello world\n42\n42.5");
}
