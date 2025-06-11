use crate::ErrorCode;
use crate::test_utils::execute_program_expect_error;

#[test]
fn basic() {
    let program = r#"
        print_value(undefined_var);
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::UndefinedVariable,
        "Undefined variable: undefined_var",
    );
}

#[test]
fn in_expression() {
    let program = r#"
        let x = 10;
        let result = x + undefined_var;
        print_value(result);
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::UndefinedVariable,
        "Undefined variable: undefined_var",
    );
}

#[test]
fn in_assignment() {
    let program = r#"
        let x = undefined_var;
        print_value(x);
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::UndefinedVariable,
        "Undefined variable: undefined_var",
    );
}

#[test]
fn in_function_call() {
    let program = r#"
        print_value(undefined_var);
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::UndefinedVariable,
        "Undefined variable: undefined_var",
    );
}
