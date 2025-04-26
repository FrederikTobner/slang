use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};

#[test]
fn test_negation_operator() {
    let program = r#"
        let a: i32 = 42;
        print_value(-a);
    "#;
    execute_program_and_assert(program, "-42");
}

#[test]
fn test_negation_with_float() {
    let program = r#"
        let a: f64 = 42.5;
        print_value(-a);
    "#;
    execute_program_and_assert(program, "-42.5");
}

#[test]
fn test_unary_on_string_error() {
    let program = r#"
        let a: string = "Hello";
        print_value(-a);
    "#;
    execute_program_expect_error(program, "Cannot negate non-numeric type");
}

#[test]
fn test_unary_negation() {
    let program = r#"
        let a: i32 = 42;
        print_value(-a);
        
        let b: f64 = 3.14;
        print_value(-b);
    "#;
    execute_program_and_assert(program, "-42");
    execute_program_and_assert(program, "-3.14");
}