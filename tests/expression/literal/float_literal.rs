use crate::test_utils::execute_program_and_assert;
use rstest::rstest;

#[test]
fn basic() {
    let program = r#"
        print_value(3.14);
    "#;
    execute_program_and_assert(program, "3.14");
}

#[rstest]
#[case("3.14f32")]
#[case("2.718f64")]
fn with_suffix(#[case] literal: &str) {
    let expected = literal.replace("f32", "").replace("f64", "");
    let program = format!(r#"print_value({});"#, literal);
    execute_program_and_assert(&program, &expected);
}

#[test]
fn scientific_notation() {
    let program = r#"
        print_value(1.23e4);
    "#;
    execute_program_and_assert(program, "12300");
}

#[test]
fn negative() {
    let program = r#"
        print_value(-3.14);
    "#;
    execute_program_and_assert(program, "-3.14");
}
