use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};

#[test]
fn test_logical_and_operator_true_true() {
    let program = r#"
        let a: bool = true;
        let b: bool = true;
        print_value(a && b);
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn test_logical_and_operator_true_false() {
    let program = r#"
        let a: bool = true;
        let b: bool = false;
        print_value(a && b);
    "#;
    execute_program_and_assert(program, "false");
}

#[test]
fn test_logical_and_operator_false_true() {
    let program = r#"
        let a: bool = false;
        let b: bool = true;
        print_value(a && b);
    "#;
    execute_program_and_assert(program, "false");
}

#[test]
fn test_logical_and_operator_false_false() {
    let program = r#"
        let a: bool = false;
        let b: bool = false;
        print_value(a && b);
    "#;
    execute_program_and_assert(program, "false");
}

#[test]
fn test_logical_or_operator_true_true() {
    let program = r#"
        let a: bool = true;
        let b: bool = true;
        print_value(a || b);
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn test_logical_or_operator_true_false() {
    let program = r#"
        let a: bool = true;
        let b: bool = false;
        print_value(a || b);
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn test_logical_or_operator_false_true() {
    let program = r#"
        let a: bool = false;
        let b: bool = true;
        print_value(a || b);
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn test_logical_or_operator_false_false() {
    let program = r#"
        let a: bool = false;
        let b: bool = false;
        print_value(a || b);
    "#;
    execute_program_and_assert(program, "false");
}

#[test]
fn test_logical_operators_with_non_boolean_types() {
    let program = r#"
        let a: i32 = 1;
        let b: bool = true;
        print_value(a && b);
    "#;
    execute_program_expect_error(program, "Compilation failed: Logical operator '&&' requires boolean operands, got i32 and bool");
}

#[test]
fn test_logical_and_short_circuit() {
    // If short-circuiting works correctly, this will not cause an error
    // because the second part won't be evaluated when the first is false
    let program = r#"
        let result = false && (1 / 0 > 0);
        print_value(result);
    "#;
    execute_program_and_assert(program, "false");
}

#[test]
fn test_logical_or_short_circuit() {
    // If short-circuiting works correctly, this will not cause an error
    // because the second part won't be evaluated when the first is true
    let program = r#"
        let result = true || (1 / 0 > 0);
        print_value(result);
    "#;
    execute_program_and_assert(program, "true");
}

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