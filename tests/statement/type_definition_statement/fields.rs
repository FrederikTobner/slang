use crate::assertions::ProgramAssertion;

#[test]
fn function_type_field() {
    // Arrange
    let program = r#"
        struct Callback {
            callback: fn(string) -> string,
        };
        print_value("struct with function type field defined");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("struct with function type field defined");
}
