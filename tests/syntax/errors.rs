use crate::test_utils::execute_program_expect_error;

#[test]
fn missing_semicolon() {
    let program = r#"
        let a = 42
        print_value(a);
    "#;
    execute_program_expect_error(program, "[E1001]", "Expected \';\' after let statement");
}

#[test]
fn mismatched_brackets() {
    let program = r#"
        fn test() {
    "#;
    execute_program_expect_error(program, "[E1002]", "Expected \'}\' after block");
}

#[test]
fn mismatched_parentheses() {
    let program = r#"
        let a = 42;
        print_value(a;
    "#;
    execute_program_expect_error(
        program,
        "[E1003]",
        "Expected \')\' after function arguments",
    );
}

#[test]
fn invalid_assignment() {
    let program = r#"
        let a = 42;
        42 = a;
    "#;
    execute_program_expect_error(program, "[E1001]", "Expected \';\' after expression");
}

#[test]
fn invalid_variable_declaration() {
    let program = r#"
        let 123abc = 42;
        print_value(123abc);
    "#;
    execute_program_expect_error(program, "[E1007]", "Expected identifier after \'let\'");
}

#[test]
fn invalid_function_declaration() {
    let program = r#"
        fn 123invalid() {
            print_value(42);
        }
    "#;
    execute_program_expect_error(program, "[E1007]", "Expected function name");
}

#[test]
fn redefined_variable() {
    let program = r#"
        let a = 42;
        let a = 43;
        print_value(a);
    "#;
    execute_program_expect_error(program, "[E2002]", "Variable \'a\' already defined");
}

