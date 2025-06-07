use crate::test_utils::execute_program_and_assert;
use rstest::rstest;

#[test]
fn unit_assignment() {
    let program = r#"
        let mut x = ();
        x = ();
        print_value(x);
    "#;
    execute_program_and_assert(program, "()");
}

// Integer types
#[rstest]
#[case("42", "i32")]
#[case("42i32", "i32")]
#[case("42i64", "i64")]
#[case("42u32", "u32")]
#[case("42u64", "u64")]
fn integer_assignment(#[case] value: &str, #[case] _type: &str) {
    let program = format!(
        r#"
        let mut x: {} = {};
        x = 12;
        print_value(x);
    "#,
        _type, value
    );
    execute_program_and_assert(&program, "12");
}

// Floating-point types
#[rstest]
#[case("3.14", "f32")]
#[case("3.14f32", "f32")]
#[case("3.14f64", "f64")]
fn float_assignment(#[case] value: &str, #[case] _type: &str) {
    let program = format!(
        r#"
        let mut x: {} = {};
        x = 2.71;
        print_value(x);
    "#,
        _type, value
    );
    execute_program_and_assert(&program, "2.71");
}

// String type
#[test]
fn string_assignment() {
    let program = r#"
        let mut x: string = "Hello";
        x = "World";
        print_value(x);
    "#;
    execute_program_and_assert(program, "World");
}

#[test]
fn boolean_assignment() {
    let program = r#"
        let mut x: bool = true;
        x = false;
        print_value(x);
    "#;
    execute_program_and_assert(program, "false");
}