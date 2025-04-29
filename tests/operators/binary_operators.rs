use crate::test_utils::{execute_program_expect_error, execute_program_and_assert};

#[test]
fn test_addition_operator() {
    let program = r#"
        let a: i32 = 15;
        let b: i32 = 27;
        print_value(a + b);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_subtraction_operator() {
    let program = r#"
        let a: i32 = 50;
        let b: i32 = 8;
        print_value(a - b);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_multiplication_operator() {
    let program = r#"
        let a: i32 = 6;
        let b: i32 = 7;
        print_value(a * b);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_division_operator() {
    let program = r#"
        let a: i32 = 126;
        let b: i32 = 3;
        print_value(a / b);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_string_concatenation() {
    let program = r#"
        let hello = "Hello, ";
        let world = "world!";
        print_value(hello + world);
    "#;
    execute_program_and_assert(program, "Hello, world!");
}

#[test]
fn test_addition_with_different_integer_types() {
    let program = r#"
        let a: i32 = 20;
        let b: i64 = 22;
        print_value(a + b);
    "#;
    execute_program_expect_error(program, "Compilation failed: Type mismatch: cannot perform Plus operation with i32 and i64\n");
}

#[test]
fn test_arithmetic_with_float_values() {
    let program = r#"
        let a: f64 = 20.5;
        let b: f64 = 21.5;
        print_value(a + b);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_arithmetic_with_integer_and_float() {
    let program = r#"
        let a: i32 = 20;
        let b: f64 = 22.0;
        print_value(a + b);
    "#;
    execute_program_expect_error(program, "Type mismatch: cannot perform Plus operation with i32 and f64");
}

#[test]
fn test_arithmetic_with_different_float_types() {
    let program = r#"
        let a: f32 = 20.5;
        let b: f64 = 21.5;
        print_value(a + b);
    "#;
    execute_program_expect_error(program, "Type mismatch: cannot perform Plus operation with f32 and f64");
}
