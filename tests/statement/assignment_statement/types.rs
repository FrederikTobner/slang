use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;
use rstest::rstest;

#[test]
fn unit_assignment() {
    // Arrange
    let program = r#"
        let mut x = ();
        x = ();
        print_value(x);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}

// Integer types
#[rstest]
#[case("42", "i32")]
#[case("42i32", "i32")]
#[case("42i64", "i64")]
#[case("42u32", "u32")]
#[case("42u64", "u64")]
fn integer_assignment(#[case] value: &str, #[case] _type: &str) {
    // Arrange
    let program = format!(
        r#"
        let mut x: {_type} = {value};
        x = 12;
        print_value(x);
    "#
    );
    
    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("12");
}

// Floating-point types
#[rstest]
#[case("3.14", "f32")]
#[case("3.14f32", "f32")]
#[case("3.14f64", "f64")]
fn float_assignment(#[case] value: &str, #[case] _type: &str) {
    // Arrange
    let program = format!(
        r#"
        let mut x: {_type} = {value};
        x = 2.71;
        print_value(x);
    "#
    );
    
    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("2.71");
}

// String type
#[test]
fn string_assignment() {
    // Arrange
    let program = r#"
        let mut x: string = "Hello";
        x = "World";
        print_value(x);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("World");
}

#[test]
fn boolean_assignment() {
    // Arrange
    let program = r#"
        let mut x: bool = true;
        x = false;
        print_value(x);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("false");
}

#[test]
fn function_to_variable() {
    // Arrange
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }

        fn subtract(a: i32, b: i32) -> i32 {
            return a - b;
        }
        
        let mut my_function = add;
        print_value(my_function(10, 20));
        my_function = subtract;
        print_value(my_function(30, 10));
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("30\n20");
}

#[test]
fn native_function_to_variable() {
    // Arrange
    let program = r#"
        let mut my_print = print_value;
        my_print("Hello from native function");
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("Hello from native function");
}

#[test]
fn with_another_type() {
    // Arrange
    let program = r#"
        let mut x: i32 = 10;
        x = "Hello"; // This should cause a type mismatch error
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch: variable assignment to variable \'x\' is i32 but expression is string");
}

