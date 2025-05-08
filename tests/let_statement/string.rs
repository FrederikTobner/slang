use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[test]
fn string_type() {
    let program = r#"
        let greeting: string = "Hello, world!";
        print_value(greeting);
    "#;
    execute_program_and_assert(program, "Hello, world!");
}

#[test]
fn string_type_inference() {
    let program = r#"
        let str = "Hello";
        print_value(str);
    "#;
    execute_program_and_assert(program, "Hello");
}

#[rstest]
#[case("true")] // Boolean literal
#[case("false")] // Boolean literal
fn from_boolean_literal(
    #[case] value: &str,
) {
    let program = format!(r#"
        let a: string = {};
        print_value(a);
    "#, value);
    execute_program_expect_error(&program, "Type mismatch: variable a is string but expression is bool");
}

#[rstest]
#[case("42", "int")] // Integer literal
#[case("42i32", "i32")] // I32 literal
#[case("42i64", "i64")] // I64 literal
#[case("42u32", "u32")] // U32 literal
#[case("42u64", "u64")] // U64 literal
fn from_integer_literal(
    #[case] value: &str,
    #[case] _type: &str,
) {
    let program = format!(r#"
        let a: string = {};
        print_value(a);
    "#, value);
    execute_program_expect_error(&program, &format!("Type mismatch: variable a is string but expression is {}", _type));
}



#[rstest]
#[case("3.14", "float")] // Float literal
#[case("3.14f32", "f32")] // F32 literal
#[case("3.14f64", "f64")] // F64 literal
fn from_float_literal(
    #[case] value: &str,
    #[case] _type: &str,
) {
    let program = format!(r#"
        let a: string = {};
        print_value(a);
    "#, value);
    execute_program_expect_error(&program, &format!("Type mismatch: variable a is string but expression is {}", _type));
}
