use crate::test_utils::{execute_program_expect_error, execute_program_and_assert};
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn test_addition_operator_on_int(
    #[case] type_name: &str,
) {
    let program = format!(
        r#"
        let a: {} = 20;
        let b: {} = 22;
        print_value(a + b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn test_addition_operator_on_float(
    #[case] type_name: &str,
) {
    let program = format!(
        r#"
        let a: {} = 20.0;
        let b: {} = 22.0;
        print_value(a + b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "42");
}

#[test]
fn test_string_concatenation() {
    let program = r#"
        let hello = "Hello, ";
        let world = "world!";
        print_value(hello + world);
    "#;
    execute_program_and_assert(program, "Hello, world!");
}

#[test]
fn test_addition_with_different_integer_types() {
    let program = r#"
        let a: i32 = 20;
        let b: i64 = 22;
        print_value(a + b);
    "#;
    execute_program_expect_error(program, "Type mismatch: cannot perform Plus operation with i32 and i64\n");
}

#[test]
fn test_arithmetic_with_i32_and_f32() {
    let program = r#"
        let a: i32 = 20;
        let b: f32 = 22.0;
        print_value(a + b);
    "#;
    execute_program_expect_error(program, "Type mismatch: cannot perform Plus operation with i32 and f32");
}

#[test]
fn test_arithmetic_with_i64_and_f64() {
    let program = r#"
        let a: i64 = 20;
        let b: f64 = 22.0;
        print_value(a + b);
    "#;
    execute_program_expect_error(program, "Type mismatch: cannot perform Plus operation with i64 and f64");
}

#[test]
fn test_arithmetic_with_integer_and_float() {
    let program = r#"
        let a = 20;
        let b = 22.0;
        print_value(a + b);
    "#;
    execute_program_expect_error(program, "Type mismatch: cannot perform Plus operation with int and float");
}

#[test]
fn test_arithmetic_with_different_float_types() {
    let program = r#"
        let a: f32 = 20.5;
        let b: f64 = 21.5;
        print_value(a + b);
    "#;
    execute_program_expect_error(program, "Type mismatch: cannot perform Plus operation with f32 and f64");
}
