use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use crate::ErrorCode;
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn equal_integer(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 5;
        let b: {} = 5;
        
        print_value(a != b);
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
fn not_equal_integer(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 5;
        let b: {} = 10;
        
        print_value(a != b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "true");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn equal_float(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 5.5;
        let b: {} = 5.5;
        
        print_value(a != b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "false");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn not_equal_float(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {} = 5.5;
        let b: {} = 10.5;
        
        print_value(a != b);
    "#,
        type_name, type_name
    );
    execute_program_and_assert(&program, "true");
}

#[test]
fn with_booleans() {
    let program = r#"
        let result1 = true != true;
        let result2 = false != false;
        let result3 = true != false;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "false\nfalse\ntrue");
}

#[test]
fn with_strings() {
    let program = r#"
        let result1 = "hello" != "hello";
        let result2 = "hello" != "world";
        
        print_value(result1);
        print_value(result2);
    "#;
    execute_program_and_assert(program, "false\ntrue");
}

#[test]
fn with_unit() {
    let program = r#"
        let x = ();
        let y = ();
        print_value(x != y);
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::OperationTypeMismatch,
        "Type mismatch: cannot apply '!=' operator on () and ()",
    );
}

#[test]
fn with_function() {
    let program = r#"
        fn my_function() {}
        let fun_1 = my_function;
        let fun_2 = my_function;
        print_value(fun_1 != fun_2);
    "#;
    execute_program_and_assert(program, "false");
}

#[test]
fn with_native_function() {
    let program = r#"
        print_value(print_value != print_value);
    "#;
    execute_program_and_assert(program, "false");
}