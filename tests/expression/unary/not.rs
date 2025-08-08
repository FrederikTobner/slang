use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;
use rstest::rstest;

#[rstest]
#[case("false", "true")]
#[case("true", "false")]
fn with_boolean_variable(#[case] input: &str, #[case] expected: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: bool = {input};
        print_value(!a);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout(expected);
}

#[rstest]
#[case("false", "true")]
#[case("true", "false")]
fn with_boolean_literal(#[case] input: &str, #[case] expected: &str) {
    // Arrange
    let program = format!("print_value(!{input});");

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout(expected);
}

#[rstest]
#[case("false")]
#[case("true")]
fn double_not_with_boolean_variable(#[case] input: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: bool = {input};
        print_value(!(!a));
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout(input);
}

#[rstest]
#[case("false")]
#[case("true")]
fn double_not_with_boolean_literal(#[case] input: &str) {
    // Arrange
    let program = format!("print_value(!(!{input}));");

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout(input);
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn with_integer(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 42;
        print_value(!a);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr(&format!(
            "Boolean not operator '!' can only be applied to boolean types, but got {type_name}"
        ));
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn with_float(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 42.0;
        print_value(!a);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr(&format!(
            "Boolean not operator '!' can only be applied to boolean types, but got {type_name}"
        ));
}

#[test]
fn with_unit() {
    // Arrange
    let program = r#"
        let x = ();
        print_value(!x);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Boolean not operator '!' can only be applied to boolean types, but got ()");
}

#[test]
fn with_function() {
    // Arrange
    let program = r#"
        fn my_function() {}
        print_value(!my_function);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr(
            "Boolean not operator '!' can only be applied to boolean types, but got fn() -> ()",
        );
}

#[test]
fn with_native_function() {
    // Arrange
    let program: &'static str = r#"
        print_value(!print_value);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Boolean not operator '!' can only be applied to boolean types, but got fn(unknown) -> i32");
}

#[test]
fn with_string() {
    // Arrange
    let program = r#"
        let a: string = "Hello";
        print_value(!a);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Boolean not operator '!' can only be applied to boolean types, but got string");
}

#[test]
fn with_string_literal() {
    // Arrange
    let program = r#"
        print_value(!"Hello");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::InvalidUnaryOperation)
        .stderr("Boolean not operator '!' can only be applied to boolean types, but got string");
}
