use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;
use rstest::rstest;

#[rstest]
#[case("false")]
#[case("true")]
fn from_boolean_literal(#[case] value: &str) {
    // Arrange
    let program = format!(
        r#"
        let boolean_var: bool = {value};
        print_value(boolean_var);
    "#
    );
    
    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout(value);
}

#[test]
fn boolean_type_inference() {
    // Arrange
    let program = r#"
        let is_true = true;
        print_value(is_true);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("true");
}

#[test]
fn from_string_literal() {
    // Arrange
    let program = r#"
        let a: bool = "Hello";
        print_value(a);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch: variable a is bool but expression is string");
}

#[rstest]
#[case("42", "int")] 
#[case("42i32", "i32")]
#[case("42i64", "i64")]
#[case("42u32", "u32")]
#[case("42u64", "u64")]
fn from_integer_literal(#[case] value: &str, #[case] _type: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: bool = {value};
        print_value(a);
    "#
    );
    
    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr(&format!(
            "Type mismatch: variable a is bool but expression is {_type}"
        ));
}

#[rstest]
#[case("3.14", "float")]
#[case("3.14f32", "f32")] 
#[case("3.14f64", "f64")] 
fn from_float_literal(#[case] value: &str, #[case] _type: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: bool = {value};
        print_value(a);
    "#
    );
    
    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr(&format!(
            "Type mismatch: variable a is bool but expression is {_type}"
        ));
}

#[test]
fn using_boolean_type_as_name() {
    // Arrange
    let program = r#"
        let bool: bool = true;
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::SymbolRedefinition)
        .stderr("Symbol \'bool\' of kind \'variable (conflicts with type)\' is already defined or conflicts with an existing symbol.");
}
