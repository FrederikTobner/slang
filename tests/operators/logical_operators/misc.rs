use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};

#[test]
fn test_logical_operators_complex_expression() {
    let program = r#"
        let a: bool = true;
        let b: bool = false;
        let c: bool = true;
        print_value(a && b || c); 
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn test_logical_operators_with_not() {
    let program = r#"
        let a: bool = true;
        let b: bool = true;
        print_value(!(a && b)); 
    "#;
    execute_program_and_assert(program, "false");
}

#[test]
fn test_logical_operators_precedence() {
    let program = r#"
        let a: bool = true;
        let b: bool = false;
        let c: bool = true; 
        let result = a && b || c;
        print_value(result);
    "#;
    execute_program_and_assert(program, "true");
}
