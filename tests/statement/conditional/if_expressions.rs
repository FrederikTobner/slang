use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn with_integer_types(#[case] type_name: &str) {
    let program = format!(r#"
        let a: {} = 50;
        let b: {} = 8;
        print_value(if a > b {{ a - b }} else {{ b - a }});
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn with_float_types(#[case] type_name: &str) {
    let program = format!(r#"
        let a: {} = 50.0;
        let b: {} = 8.0;
        print_value(if a > b {{ a - b }} else {{ b - a }});
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}

#[test]
fn with_strings() {
    let program = r#"
        let x: i32 = 5;
        let result: string = if x > 3 { "greater" } else { "lesser" };
        print_value(result);
    "#;
    execute_program_and_assert(program, "greater");
}

#[test]
fn with_booleans() {
    let program = r#"
        let x: i32 = 5;
        let result: bool = if x > 3 { true } else { false };
        print_value(result);
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn nested() {
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 3;
        let result: i32 = if x > y { 
            if x > 10 { 100 } else { 50 } 
        } else { 
            if y > 10 { 200 } else { 25 } 
        };
        print_value(result);
    "#;
    execute_program_and_assert(program, "50");
}

#[test]
fn in_function_call() {
    let program = r#"
        let x: i32 = 5;
        print_value(if x > 3 { "true" } else { "false" });
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn with_complex_condition() {
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 3;
        let result: string = if x > y && x < 10 { "in range" } else { "out of range" };
        print_value(result);
    "#;
    execute_program_and_assert(program, "in range");
}

#[test]
fn type_mismatch() {
    let program = r#"
        let x: i32 = 5;
        let result: i32 = if x > 3 { 10 } else { "string" };
        print_value(result);
    "#;
    execute_program_expect_error(program, "[E2005]", "Type mismatch");
}

#[test]
fn non_boolean_condition() {
    let program = r#"
        let x: i32 = 5;
        let result: i32 = if x { 10 } else { 20 };
        print_value(result);
    "#;
    execute_program_expect_error(program, "[E2005]", "Type mismatch");
}

#[test]
fn with_arithmetic() {
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 3;
        let result: i32 = if x > y { x + y } else { x - y };
        print_value(result);
    "#;
    execute_program_and_assert(program, "8");
}

#[test]
fn chained() {
    let program = r#"
        let x: i32 = 5;
        let a: i32 = if x > 3 { 10 } else { 5 };
        let b: i32 = if a > 7 { 20 } else { 15 };
        print_value(b);
    "#;
    execute_program_and_assert(program, "20");
}

#[test]
fn must_have_same_type() {
    let program = r#"
        let x: i32 = 5;
        let result: i32 = if x > 3 { 10 } else { "string" };
        print_value(result);
    "#;
    execute_program_expect_error(program, "[E2005]", "Type mismatch");
}
#[test]
fn must_have_else_branch() {
    let program = r#"
        let x: i32 = 5;
        let result: i32 = if x > 3 { 10 };
        print_value(result);
    "#;
    execute_program_expect_error(program, "[E1031]", "Expected 'else' after if expression");
}