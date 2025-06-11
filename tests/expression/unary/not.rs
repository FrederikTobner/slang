use crate::ErrorCode;
use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("false", "true")]
#[case("true", "false")]
fn with_boolean_variable(#[case] input: &str, #[case] expected: &str) {
    let program = format!(
        r#"
        let a: bool = {};
        print_value(!a);
    "#,
        input
    );
    execute_program_and_assert(&program, expected);
}

#[rstest]
#[case("false", "true")]
#[case("true", "false")]
fn with_boolean_literal(#[case] input: &str, #[case] expected: &str) {
    let program = format!("print_value(!{});", input);
    execute_program_and_assert(&program, expected);
}

#[rstest]
#[case("false")]
#[case("true")]
fn double_not_with_boolean_variable(#[case] input: &str) {
    let program = format!(
        r#"
        let a: bool = {};
        print_value(!(!a));
    "#,
        input
    );
    execute_program_and_assert(&program, input);
}

#[rstest]
#[case("false")]
#[case("true")]
fn double_not_with_boolean_literal(#[case] input: &str) {
    let program = format!("print_value(!(!{}));", input);
    execute_program_and_assert(&program, input);
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn with_integer(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 42;
        print_value(!a);
    "#,
        type_name
    );
    execute_program_expect_error(
        &program,
        ErrorCode::InvalidUnaryOperation,
        &format!(
            "Boolean not operator '!' can only be applied to boolean types, but got {}",
            type_name
        ),
    );
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn with_float(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 42.0;
        print_value(!a);
    "#,
        type_name
    );
    execute_program_expect_error(
        &program,
        ErrorCode::InvalidUnaryOperation,
        &format!(
            "Boolean not operator '!' can only be applied to boolean types, but got {}",
            type_name
        ),
    );
}

#[test]
fn with_unit() {
    let program = r#"
        let x = ();
        print_value(!x);
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::InvalidUnaryOperation,
        "Boolean not operator '!' can only be applied to boolean types, but got ()",
    );
}

#[test]
fn with_function() {
    let program = r#"
        fn my_function() {}
        print_value(!my_function);
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::InvalidUnaryOperation,
        "Boolean not operator '!' can only be applied to boolean types, but got fn() -> ()",
    );
}

#[test]
fn with_native_function() {
    let program: &'static str = r#"
        print_value(!print_value);
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::InvalidUnaryOperation,
        "Boolean not operator '!' can only be applied to boolean types, but got fn(unknown) -> i32",
    );
}

#[test]
fn with_string() {
    let program = r#"
        let a: string = "Hello";
        print_value(!a);
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::InvalidUnaryOperation,
        "Boolean not operator '!' can only be applied to boolean types, but got string",
    );
}

#[test]
fn with_string_literal() {
    let program = r#"
        print_value(!"Hello");
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::InvalidUnaryOperation,
        "Boolean not operator '!' can only be applied to boolean types, but got string",
    );
}

