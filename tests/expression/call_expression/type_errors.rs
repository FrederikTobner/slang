use crate::ErrorCode;
use crate::assertions::ProgramAssertion;
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("u32")]
#[case("i64")]
#[case("u64")]
fn with_integer_variable(#[case] type_name: &str) {
    // Arrange
    let program = format!("let a: {type_name} = 42; a();");

    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::VariableNotCallable)
        .stderr(&format!("Cannot call {type_name} type 'a' as a function"));
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn with_float_variable(#[case] type_name: &str) {
    // Arrange
    let program = format!("let a: {type_name} = 42.0; a();");

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

#[test]
fn argument_count_mismatch() {
    // Arrange
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }

        add(1);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ArgumentCountMismatch)
        .stderr("Function 'add' expects 2 arguments, but got 1");
}

#[test]
fn argument_type_mismatch() {
    // Arrange
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }

        add("hello", 5);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ArgumentTypeMismatch)
        .stderr("Type mismatch: function 'add' expects argument 1 to be i32, but got string");
}
