use crate::assertions::ProgramAssertion;

#[test]
fn complex_expression() {
    // Arrange
    let program = r#"
        let a: bool = true;
        let b: bool = false;
        let c: bool = true;
        print_value(a && b || c); 
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("true");
}

#[test]
fn with_not() {
    // Arrange
    let program = r#"
        let a: bool = true;
        let b: bool = true;
        print_value(!(a && b)); 
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("false");
}

#[test]
fn precedence() {
    // Arrange
    let program = r#"
        let a: bool = true;
        let b: bool = false;
        let c: bool = true; 
        let result = a && b || c;
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("true");
}
