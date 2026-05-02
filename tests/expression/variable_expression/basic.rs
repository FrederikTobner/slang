use crate::test_utils::ProgramAssertion;

#[test]
fn simple_variable_reference() {
    // Arrange
    let program = r#"
        let x = 42;
        print_value(x);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn string_variable_reference() {
    // Arrange
    let program = r#"
        let message = "hello";
        print_value(message);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("hello");
}

#[test]
fn boolean_variable_reference() {
    // Arrange
    let program = r#"
        let flag = true;
        print_value(flag);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("true");
}

#[test]
fn unit_variable_reference() {
    // Arrange
    let program = r#"
        let unit_value = ();
        print_value(unit_value);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}

#[test]
fn function_variable_reference() {
    // Arrange
    let program = r#"
        fn greet() -> string {
            return "hello";
        }
        
        let greeting = greet;
        print_value(greeting);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("<fn greet>");
}

#[test]
fn native_function_reference() {
    // Arrange
    let program = r#"
        let native_print = print_value;
        native_print(native_print);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("<native fn print_value>");
}
