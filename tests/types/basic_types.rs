use crate::test_utils::execute_program_and_assert;
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
#[test]
fn integer_type(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = 42;
        print_value(a);
    "#, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn float_type(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = 42.5;
        print_value(a);
    "#, type_name);
    execute_program_and_assert(&program, "42.5");
}

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

#[test]
fn integer_type_inference() {
    let program = r#"
        let a = 42;
        print_value(a);
    "#;
    execute_program_and_assert(program, "42");
}

#[rstest]
#[case("false")]
#[case("true")]
fn boolean_literal(#[case] value: &str,) {
    let program = format!(r#"
        let boolean_var: bool = {};
        print_value(boolean_var);
    "#, value);
    execute_program_and_assert(&program, value);
}


#[test]
fn boolean_type_inference() {
    let program = r#"
        let is_true = true;
        print_value(is_true);
    "#;
    execute_program_and_assert(program, "true");
}

