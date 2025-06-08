use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn smaller_on_int(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 20;
        let b: {} = 22;
        print_value(a >= b);
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
fn equal_on_int(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 20;
        let b: {} = 20;
        print_value(a >= b);
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
fn greater_on_int(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 22;
        let b: {} = 20;
        print_value(a >= b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "true");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn smaller_on_float(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 20.0;
        let b: {} = 22.0;
        print_value(a >= b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "false");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn equal_on_float(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 20.0;
        let b: {} = 20.0;
        print_value(a >= b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "true");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn greater_on_float(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 22.0;
        let b: {} = 20.0;
        print_value(a >= b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "true");
}

#[test]
fn with_unit() {
    let program = r#"
        let x = ();
        let y = ();
        print_value(x >= y);
    "#;
    execute_program_expect_error(
        program,
        "[E2006]",
        "Type mismatch: cannot apply '>=' operator on () and ()",
    );
}

#[test]
fn with_booleans() {
    let program = r#"
        let result1 = true >= true;
    "#;
    execute_program_expect_error(
        program,
        "[E2006]",
        "Type mismatch: cannot apply '>=' operator on bool and bool",
    );
}

#[test]
fn with_strings() {
    let program = r#"
        let result1 = "hello" >= "hello";
    "#;
    execute_program_expect_error(
        program,
        "[E2006]",
        "Type mismatch: cannot apply '>=' operator on string and string",
    );
}

#[test]
fn with_function() {
    let program = r#"
        fn my_function() {}
        let fun_1 = my_function;
        let fun_2 = my_function;
        print_value(fun_1 >= fun_2);
    "#;
    execute_program_expect_error(
        program,
        "[E2006]",
        "Type mismatch: cannot apply '>=' operator on fn() -> () and fn() -> ()",
    );
}

#[test]
fn with_native_function() {
    let program = r#"
        print_value(print_value >= print_value);
    "#;
    execute_program_expect_error(
        program,
        "[E2006]",
        "Type mismatch: cannot apply '>=' operator on fn(unknown) -> i32 and fn(unknown) -> i32",
    );
}