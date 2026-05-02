use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn with_integer_variable() {
    // Arrange
    let program = r#"
        let a: i32 = 42;
        print_value(-a);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("-42");
}

#[test]
fn with_int_literal() {
    // Arrange
    let program = "print_value(-42);";

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("-42");
}

#[test]
fn with_float_variable() {
    // Arrange
    let program = r#"
        let a: f64 = 42.5;
        print_value(-a);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("-42.5");
}

#[test]
fn with_float_literal() {
    // Arrange
    let program = "print_value(-42.5);";

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("-42.5");
}

#[test]
fn with_string() {
    // Arrange
    let program = r#"
        let a: string = "Hello";
        print_value(-a);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Cannot negate non-numeric type \'string\'");
}

#[test]
fn with_string_literal() {
    // Arrange
    let program = r#"
        print_value(-"Hello");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Cannot negate non-numeric type 'string'");
}

#[test]
fn with_unsigned_integer() {
    // Arrange
    let program = r#"
        let a: u32 = 42;
        print_value(-a);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Cannot negate unsigned type");
}

#[test]
fn double_negation() {
    // Arrange
    let program = r#"
        let a: i32 = 42;
        print_value(-(-a));
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn with_unit() {
    // Arrange
    let program = r#"
        let x = ();
        print_value(-x);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Cannot negate non-numeric type '()'");
}

#[test]
fn with_function() {
    // Arrange
    let program = r#"
        fn my_function() -> i32 {
            42
        }
        print_value(-my_function);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Cannot negate non-numeric type 'fn() -> i32'");
}

#[test]
fn with_native_function() {
    // Arrange
    let program = r#"
        print_value(-print_value);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Cannot negate non-numeric type 'fn(unknown) -> i32'");
}

#[test]
fn with_boolean() {
    // Arrange
    let program = r#"
        let a: bool = true;
        print_value(-a);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Cannot negate non-numeric type 'bool'");
}

#[test]
fn with_boolean_literal() {
    // Arrange
    let program = r#"
        print_value(-true);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Cannot negate non-numeric type 'bool'");
}
