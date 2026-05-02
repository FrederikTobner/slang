use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;
use rstest::rstest;

#[rstest]
#[case("i32", "42")]
#[case("u32", "42")]
#[case("i64", "42")]
#[case("u64", "42")]
fn with_integer_variable(#[case] type_name: &str, #[case] value: &str) {
    // Arrange
    let program = format!("let a: {type_name} = {value}; a();");

    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::VariableNotCallable)
        .stderr(&format!("Cannot call {type_name} type 'a' as a function"));
}

#[rstest]
#[case("f32", "42.0")]
#[case("f64", "42.0")]
fn with_float_variable(#[case] type_name: &str, #[case] value: &str) {
    // Arrange
    let program = format!("let a: {type_name} = {value}; a();");

    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::VariableNotCallable)
        .stderr(&format!("Cannot call {type_name} type 'a' as a function"));
}

#[test]
fn with_string_variable() {
    // Arrange
    let program = r#"
        let a: string = "Hello";
        a();
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(crate::ErrorCode::VariableNotCallable)
        .stderr("Cannot call string type 'a' as a function");
}

#[rstest]
#[case("true")]
#[case("false")]
fn with_boolean_variable(#[case] value: &str) {
    // Arrange
    let program = format!("let a: bool = {value}; a();");

    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::VariableNotCallable)
        .stderr("Cannot call bool type 'a' as a function");
}

#[test]
fn with_unit_variable() {
    // Arrange
    let program = r#"
        let a = ();
        a();
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::VariableNotCallable)
        .stderr("Cannot call () type 'a' as a function");
}
