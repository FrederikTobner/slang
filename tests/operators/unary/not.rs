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
        "[E2015]",
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
        "[E2015]",
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
        "[E2015]",
        "Boolean not operator '!' can only be applied to boolean types, but got ()",
    );
}
