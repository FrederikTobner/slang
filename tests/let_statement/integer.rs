
use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("")]
#[case(": i32")]
#[case(": i64")]
#[case(": u32")]
#[case(": u64")]
fn from_literal(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a{} = 42;
        print_value(a);
    "#, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_literal_with_type_suffix(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a = 42{};
        print_value(a);
    "#, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("")] // No type suffix
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_binary_expression (
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a = 20{} + 22{};
        print_value(a);
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_true_literal(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = true;
    "#, type_name);
    execute_program_expect_error(&program, &format!("Type mismatch: variable a is {} but expression is bool", type_name));
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_false_literal(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = false;
    "#, type_name);
    execute_program_expect_error(&program, &format!("Type mismatch: variable a is {} but expression is bool", type_name));
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_string_literal(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = "hello";
    "#, type_name);
    execute_program_expect_error(&program, &format!("Type mismatch: variable a is {} but expression is string", type_name));
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_float_literal(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = 42.0;
    "#, type_name);
    execute_program_expect_error(&program, &format!("Type mismatch: variable a is {} but expression is float", type_name));
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_float_literal_with_f32_suffix(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = 42.0f32;
    "#, type_name);
    execute_program_expect_error(&program, &format!("Type mismatch: variable a is {} but expression is f32", type_name));
}


#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_float_literal_with_f64_suffix(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = 42.0f64;
    "#, type_name);
    execute_program_expect_error(&program, &format!("Type mismatch: variable a is {} but expression is f64", type_name));
}


