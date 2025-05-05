use crate::test_utils::execute_program_and_assert;
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

