use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};

#[test]
fn test_boolean_not() {
    let program = r#"
        let a: bool = true;
        print_value(!a);
    "#;
    execute_program_and_assert(program, "false");
}

#[test]
fn test_boolean_not_with_literal() {
    let program = r#"
        print_value(!false);
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn test_double_boolean_not() {
    let program = r#"
        let a: bool = true;
        print_value(!(!a));
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn test_boolean_not_type_error() {
    let program = r#"
        let a: i32 = 42;
        print_value(!a);
    "#;
    execute_program_expect_error(program, "Boolean negation operator '!' can only be applied to boolean types");
}

