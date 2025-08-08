use crate::test_utils::ProgramAssertion;
use rstest::rstest;
use slang_error::ErrorCode;

#[test]
fn unit_literal() {
    // Arrange
    let program = r#"
        let x = ();
        print_value(x);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}

#[test]
fn unit_type_annotation() {
    // Arrange
    let program = r#"
        let x: () = ();
        print_value(x);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}

#[rstest]
#[case("true")]
#[case("false")]
fn boolean_literal(#[case] value: &str) {
    // Arrange
    let program = format!(r#"
        let x : () = {value};
    "#);
    
    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr(" Type mismatch: variable x is () but expression is bool");
}

#[rstest]
#[case("42", "int")]
#[case("42i32", "i32")]
#[case("42i64", "i64")]
#[case("42u32", "u32")]
#[case("42u64", "u64")]
fn integer_literal(#[case] value: &str, #[case] used_type: &str) {
    // Arrange
    let program = format!(r#"
        let x : () = {value};
    "#);
    
    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr(&format!(" Type mismatch: variable x is () but expression is {used_type}"));
}

#[rstest]
#[case("3.14", "float")]
#[case("3.14f32", "f32")]
#[case("3.14f64", "f64")]
fn float_literal(#[case] value: &str, #[case] used_type: &str) {
    // Arrange
    let program = format!(r#"
        let x : () = {value};
    "#);
    
    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr(&format!(" Type mismatch: variable x is () but expression is {used_type}"));
}

#[test]
fn string_literal() {
    // Arrange
    let program = r#"
        let x : () = "Hello, world!";
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr(" Type mismatch: variable x is () but expression is string");
}