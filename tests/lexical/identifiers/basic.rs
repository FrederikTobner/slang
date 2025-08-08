use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn valid_identifier() {
    // Arrange
    let program = r#"
        let valid_name = 42;
        print_value(valid_name);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn identifier_with_underscore() {
    // Arrange
    let program = r#"
        let private_var = "hidden";
        let my_var_name = "visible";
        print_value(private_var);
        print_value(my_var_name);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("hidden\nvisible");
}

#[test]
fn identifier_with_numbers() {
    // Arrange
    let program = r#"
        let var1 = 10;
        let var2name = 20;
        let name3var = 30;
        print_value(var1);
        print_value(var2name);
        print_value(name3var);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("10\n20\n30");
}

#[test]
fn starting_with_number_error() {
    // Arrange
    let program = r#"
        let 1invalid = 42;
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedIdentifier)
        .stderr("Expected identifier");
}

#[test]
fn with_special_characters_error() {
    // Arrange
    let program = r#"
        let invalid-name = 42;
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedEquals)
        .stderr("Expected '='");
}
