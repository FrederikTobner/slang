use crate::ErrorCode;
use crate::assertions::ProgramAssertion;
use rstest::rstest;

#[rstest]
#[case("true", "true", "true")]
#[case("true", "false", "true")]
#[case("false", "true", "true")]
#[case("false", "false", "false")]
fn with_boolean_variables(#[case] first: &str, #[case] second: &str, #[case] expected: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: bool = {first};
        let b: bool = {second};
        print_value(a || b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout(expected);
}

#[rstest]
#[case("true", "true", "true")]
#[case("true", "false", "true")]
#[case("false", "true", "true")]
#[case("false", "false", "false")]
fn with_boolean_literals(#[case] first: &str, #[case] second: &str, #[case] expected: &str) {
    // Arrange
    let program = format!("print_value({first} || {second});");

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout(expected);
}

#[test]
fn with_non_boolean_types() {
    // Arrange
    let program = r#"
        let a: i32 = 1;
        let b: bool = true;
        print_value(a || b);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::LogicalOperatorTypeMismatch)
        .stderr("Logical operator '||' requires boolean operands, got i32 and bool");
}

#[test]
fn short_circuit() {
    // Arrange
    // If short-circuiting works correctly, this will not cause an error
    // because the second part won't be evaluated when the first is true
    let program = r#"
        let result = true || (1 / 0 > 0);
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("true");
}

#[test]
fn with_function() {
    // Arrange
    let program = r#"
        fn my_function() {}
        print_value(my_function || my_function);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::LogicalOperatorTypeMismatch)
        .stderr("Logical operator \'||\' requires boolean operands, got fn() -> () and fn() -> ()");
}

#[test]
fn with_native_function() {
    // Arrange
    let program = r#"
        print_value || print_value;
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::LogicalOperatorTypeMismatch)
        .stderr("Logical operator \'||\' requires boolean operands, got fn(unknown) -> i32 and fn(unknown) -> i32");
}
