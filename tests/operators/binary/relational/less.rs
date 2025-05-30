use crate::test_utils::execute_program_and_assert;
use rstest::rstest;


#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn smaller_on_int(
    #[case] type_name: &str,
) {
    let program = format!(
        r#"
        let a: {} = 20;
        let b: {} = 22;
        print_value(a < b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "true");
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn equal_on_int(
    #[case] type_name: &str,
) {
    let program = format!(
        r#"
        let a: {} = 20;
        let b: {} = 20;
        print_value(a < b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "false");
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn greater_on_int(
    #[case] type_name: &str,
) {
    let program = format!(
        r#"
        let a: {} = 22;
        let b: {} = 20;
        print_value(a < b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "false");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn smaller_on_float(
    #[case] type_name: &str,
) {
    let program = format!(
        r#"
        let a: {} = 20.0;
        let b: {} = 22.0;
        print_value(a < b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "true");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn equal_on_float(
    #[case] type_name: &str,
) {
    let program = format!(
        r#"
        let a: {} = 20.0;
        let b: {} = 20.0;
        print_value(a < b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "false");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn greater_on_float(
    #[case] type_name: &str,
) {
    let program = format!(
        r#"
        let a: {} = 22.0;
        let b: {} = 20.0;
        print_value(a < b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "false");
}

