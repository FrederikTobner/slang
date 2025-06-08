use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("true", "true", "true")]
#[case("true", "false", "false")]
#[case("false", "true", "false")]
#[case("false", "false", "false")]
fn with_boolean_types(#[case] first: &str, #[case] second: &str, #[case] expected: &str) {
    let program = format!(
        r#"
        let a: bool = {};
        let b: bool = {};
        print_value(a && b);
    "#,
        first, second
    );
    execute_program_and_assert(&program, expected);
}

#[rstest]
#[case("true", "true", "true")]
#[case("true", "false", "false")]
#[case("false", "true", "false")]
#[case("false", "false", "false")]
fn with_boolean_literals(#[case] first: &str, #[case] second: &str, #[case] expected: &str) {
    let program = format!("print_value({} && {});", first, second);

    execute_program_and_assert(&program, expected);
}

#[test]
fn with_non_boolean_types() {
    let program = r#"
        let a: i32 = 1;
        let b: bool = true;
        print_value(a && b);
    "#;
    execute_program_expect_error(
        program,
        "[E2007]",
        "Logical operator '&&' requires boolean operands, got i32 and bool",
    );
}

#[test]
fn short_circuit() {
    // If short-circuiting works correctly, this will not cause an error
    // because the second part won't be evaluated when the first is false
    let program = r#"
        let result = false && (1 / 0 > 0);
        print_value(result);
    "#;
    execute_program_and_assert(program, "false");
}

#[test]
fn with_function() {
    let program = r#"
        fn my_function() {}
        print_value(my_function && my_function);
    "#;
    execute_program_expect_error(
        program,
        "[E2007]",
        "Logical operator \'&&\' requires boolean operands, got fn() -> () and fn() -> ()",
    );
}

#[test]
fn with_native_function() {
    let program = r#"
        print_value && print_value;
    "#;
    execute_program_expect_error(
        program,
        "[E2007]",
        "Logical operator \'&&\' requires boolean operands, got fn(unknown) -> i32 and fn(unknown) -> i32",
    );
}