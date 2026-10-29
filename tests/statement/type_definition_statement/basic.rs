use crate::assertions::ProgramAssertion;

#[test]
fn simple_type() {
    // Arrange
    let program = r#"
        struct MyStruct {
            field1: string,
            field2: i32,
        };
        print_value("struct defined successfully");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("struct defined successfully");
}

#[test]
fn empty_type() {
    // Arrange
    let program = r#"
        struct EmptyStruct {};
        print_value("empty struct defined");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("empty struct defined");
}

#[test]
fn single_field_concept() {
    // Arrange
    let program = r#"
        struct SingleFieldStruct {
            field: i32,
        };
        print_value("single field struct defined");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("single field struct defined");
}
