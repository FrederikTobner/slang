use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
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

#[test]
fn function_to_variable() {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }

        fn subtract(a: i32, b: i32) -> i32 {
            return a - b;
        }
        
        let mut my_function = add;
        print_value(my_function(10, 20));
        my_function = subtract;
        print_value(my_function(30, 10));
    "#;
    execute_program_and_assert(program, "30\n20");
}

#[test]
fn native_function_to_variable() {
    let program = r#"
        let mut my_print = print_value;
        my_print("Hello from native function");
    "#;
    execute_program_and_assert(program, "Hello from native function");
}

#[test]
fn with_another_type() {
    let program = r#"
        let mut x: i32 = 10;
        x = "Hello"; // This should cause a type mismatch error
    "#;
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable assignment to variable \'x\' is i32 but expression is string");
}