use crate::test_utils::ProgramAssertion;

#[test]
fn without_return() {
    // Arrange
    let program = r#"
        let result = {
            let x = 42;
            let y = x + 1;
        };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}
