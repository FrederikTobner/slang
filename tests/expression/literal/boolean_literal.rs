use crate::assertions::ProgramAssertion;

#[test]
fn true_literal() {
    // Arrange
    let program = r#"
        print_value(true);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("true");
}

#[test]
fn false_literal() {
    // Arrange
    let program = r#"
        print_value(false);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("false");
}
