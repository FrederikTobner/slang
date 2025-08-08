use crate::test_utils::ProgramAssertion;
use rstest::rstest;

#[test]
fn basic() {
    // Arrange
    let program = r#"
        print_value(3.14);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("3.14");
}

#[rstest]
#[case("3.14f32")]
#[case("2.718f64")]
fn with_suffix(#[case] literal: &str) {
    // Arrange
    let expected = literal.replace("f32", "").replace("f64", "");
    let program = format!(r#"print_value({literal});"#);

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout(&expected);
}

#[test]
fn scientific_notation() {
    // Arrange
    let program = r#"
        print_value(1.23e4);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("12300");
}

#[test]
fn negative() {
    // Arrange
    let program = r#"
        print_value(-3.14);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("-3.14");
}
