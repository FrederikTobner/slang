use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};

#[test]
fn with_integer_variable() {
    let program = r#"
        let a: i32 = 42;
        print_value(-a);
    "#;
    execute_program_and_assert(program, "-42");
}

#[test]
fn with_int_literal() {
    let program = "print_value(-42);";
    execute_program_and_assert(program, "-42");
}

#[test]
fn with_float_variable() {
    let program = r#"
        let a: f64 = 42.5;
        print_value(-a);
    "#;
    execute_program_and_assert(program, "-42.5");
}

#[test]
fn with_float_literal() {
    let program = "print_value(-42.5);";
    execute_program_and_assert(program, "-42.5");
}

#[test]
fn with_string() {
    let program = r#"
        let a: string = "Hello";
        print_value(-a);
    "#;
    execute_program_expect_error(
        program,
        "[E2015]",
        "Cannot negate non-numeric type \'string\'",
    );
}

#[test]
fn with_unsigned_integer() {
    let program = r#"
        let a: u32 = 42;
        print_value(-a);
    "#;
    execute_program_expect_error(program, "[E2015]", "Cannot negate unsigned type");
}

#[test]
fn double_negation() {
    let program = r#"
        let a: i32 = 42;
        print_value(-(-a));
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn with_unit() {
    let program = r#"
        let x = ();
        print_value(-x);
    "#;
    execute_program_expect_error(program, "[E2015]", "Cannot negate non-numeric type '()'");
}
