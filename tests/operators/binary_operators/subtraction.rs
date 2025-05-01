use crate::test_utils::execute_program_and_assert;
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn test_subtraction_operator_on_int(#[case] type_name: &str) {
    let program = format!(r#"
        let a: {} = 50;
        let b: {} = 8;
        print_value(a - b);
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn test_subtraction_operator_on_float(#[case] type_name: &str) {
    let program = format!(r#"
        let a: {} = 50.0;
        let b: {} = 8.0;
        print_value(a - b);
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}