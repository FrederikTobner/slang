use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn with_integer_types(#[case] type_name: &str) {
    let program = format!(r#"
        let a: {} = 6;
        let b: {} = 7;
        print_value(a * b);
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn with_float_types(#[case] type_name: &str) {
    let program = format!(r#"
        let a: {} = 6.0;
        let b: {} = 7.0;
        print_value(a * b);
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}

#[test]
fn with_string_and_integer() {
    let program = r#"
        let a = "string";
        let b = a * 3;
    "#;
    execute_program_expect_error(program, "Type mismatch: cannot perform Multiply operation with string and int\n");
}


