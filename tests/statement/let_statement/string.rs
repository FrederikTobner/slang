use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;
use rstest::rstest;

#[test]
fn string_type() {
    // Arrange
    let program = r#"
        let greeting: string = "Hello, world!";
        print_value(greeting);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("Hello, world!");
}

#[test]
fn string_type_inference() {
    // Arrange
    let program = r#"
        let str = "Hello";
        print_value(str);
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("Hello");
}

#[rstest]
#[case("true")] // Boolean literal
#[case("false")] // Boolean literal
fn from_boolean_literal(#[case] value: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: string = {value};
        print_value(a);
    "#
    );
    
    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch: variable a is string but expression is bool");
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
        let a: string = {value};
        print_value(a);
    "#
    );
    
    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr(&format!(
            "Type mismatch: variable a is string but expression is {_type}"
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
        let a: string = {value};
        print_value(a);
    "#
    );
    
    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr(&format!(
            "Type mismatch: variable a is string but expression is {_type}"
        ));
}

#[test]
fn using_string_type_as_name() {
    // Arrange
    let program = r#"
        let string: bool = true;
    "#;
    
    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::SymbolRedefinition)
        .stderr("Symbol \'string\' of kind \'variable (conflicts with type)\' is already defined or conflicts with an existing symbol.");
}
