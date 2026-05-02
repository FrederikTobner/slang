use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;
use rstest::rstest;

#[test]
fn basic() {
    // Arrange
    let program = r#"
        print_value(42);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[rstest]
#[case("42i32")]
#[case("42i64")]
#[case("42u32")]
#[case("42u64")]
fn with_suffix(#[case] literal: &str) {
    // Arrange
    let program = format!(r#"print_value({literal});"#);

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("42");
}

#[test]
fn negative() {
    // Arrange
    let program = r#"
        print_value(-42);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("-42");
}

#[test]
fn integer_overflow_error() {
    // Arrange
    let program = r#"
        print_value(999999999999999999999999999999);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidNumberLiteral)
        .stderr("Invalid integer");
}
