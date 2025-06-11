use crate::ErrorCode;
use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[test]
fn basic() {
    let program = r#"
        print_value(42);
    "#;
    execute_program_and_assert(program, "42");
}

#[rstest]
#[case("42i32")]
#[case("42i64")]
#[case("42u32")]
#[case("42u64")]
fn with_suffix(#[case] literal: &str) {
    let program = format!(r#"print_value({});"#, literal);
    execute_program_and_assert(&program, "42");
}

#[test]
fn negative() {
    let program = r#"
        print_value(-42);
    "#;
    execute_program_and_assert(program, "-42");
}

#[test]
fn integer_overflow_error() {
    let program = r#"
        print_value(999999999999999999999999999999);
    "#;
    execute_program_expect_error(program, ErrorCode::InvalidNumberLiteral, "Invalid integer");
}
