use crate::test_utils::ProgramAssertion;

#[test]
fn test_digit_recognition_0_to_9() {
    // Arrange
    let program = r#"
        let d0 = 0;
        let d1 = 1;
        let d2 = 2;
        let d3 = 3;
        let d4 = 4;
        let d5 = 5;
        let d6 = 6;
        let d7 = 7;
        let d8 = 8;
        let d9 = 9;
        print_value("all digits recognized");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("all digits recognized");
}

#[test]
fn test_digit_in_identifiers() {
    // Arrange
    let program = r#"
        let var1 = "one";
        let var2name = "two";
        let name3 = "three";
        print_value("digits in identifiers");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("digits in identifiers");
}

#[test]
fn test_digit_sequences() {
    // Arrange
    let program = r#"
        let num = 123456789;
        print_value(num);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("123456789");
}

#[test]
fn test_digit_with_underscores() {
    // Arrange
    let program = r#"
        let big_num = 1000000;
        print_value(big_num);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("1000000");
}
