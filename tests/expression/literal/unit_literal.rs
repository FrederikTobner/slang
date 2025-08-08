use crate::test_utils::ProgramAssertion;

#[test]
fn basic() {
    // Arrange
    let program = r#"
        print_value(());
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("()");
}

