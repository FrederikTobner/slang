use crate::ErrorCode;
use crate::test_utils::execute_program_expect_error;
use rstest::rstest;

#[rstest]
#[case("i32", "42")]
#[case("u32", "42")]
#[case("i64", "42")]
#[case("u64", "42")]
fn with_integer_variable(#[case] type_name: &str, #[case] value: &str) {
    let program = format!("let a: {type_name} = {value}; a();");
    execute_program_expect_error(
        &program,
        ErrorCode::VariableNotCallable,
        &format!("Cannot call {type_name} type 'a' as a function"),
    );
}

#[rstest]
#[case("f32", "42.0")]
#[case("f64", "42.0")]
fn with_float_variable(#[case] type_name: &str, #[case] value: &str) {
    let program = format!("let a: {type_name} = {value}; a();");
    execute_program_expect_error(
        &program,
        ErrorCode::VariableNotCallable,
        &format!("Cannot call {type_name} type 'a' as a function"),
    );
}

#[test]
fn with_string_variable() {
    let program = r#"
        let a: string = "Hello";
        a();
    "#;
    execute_program_expect_error(
        program,
        crate::ErrorCode::VariableNotCallable,
        "Cannot call string type 'a' as a function",
    );
}

#[rstest]
#[case("true")]
#[case("false")]
fn with_boolean_variable(#[case] value: &str) {
    let program = format!("let a: bool = {value}; a();");
    execute_program_expect_error(
        &program,
        ErrorCode::VariableNotCallable,
        "Cannot call bool type 'a' as a function",
    );
}

#[test]
fn with_unit_variable() {
    let program = r#"
        let a = ();
        a();
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::VariableNotCallable,
        "Cannot call () type 'a' as a function",
    );
}

