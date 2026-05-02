use crate::test_utils::ProgramAssertion;

#[test]
fn basic() {
    // Arrange
    let program = r#"
        let héllo = "unicode";
        print_value(héllo);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("unicode");
}

#[test]
fn emoji_identifier() {
    // Arrange
    let program = r#"
        let 😮 = "test";
        print_value(😮);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("test");
}

#[test]
fn greek_identifier() {
    // Arrange
    let program = r#"
        let π = 3.14159;
        print_value(π);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("3.14159");
}

#[test]
fn mixed_unicode_ascii() {
    // Arrange
    let program = r#"
        let user_名前 = "name";
        print_value(user_名前);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("name");
}
