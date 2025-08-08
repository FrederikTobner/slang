use crate::test_utils::ProgramAssertion;

#[test]
fn precedence() {
    // Arrange
    let program = r#"
        let a: i32 = 1;
        let b: i32 = 2;
        let c: i32 = 3;
        let result = a + b * c;
        print_value(result);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("7");
}
