use crate::test_utils::execute_program_expect_error;

#[test]
fn test_missing_semicolon() {
    let program = r#"
        let a = 42
        print_value(a);
    "#;
    execute_program_expect_error(program, "Expected \';\' after let statement");
}

#[test]
fn test_mismatched_brackets() {
    let program = r#"
        fn test() {
    "#;
    execute_program_expect_error(program, "Expected \'}\' after function body");
}

#[test]
fn test_mismatched_parentheses() {
    let program = r#"
        let a = 42;
        print_value(a;
    "#;
    execute_program_expect_error(program, "Expected \')\' after function arguments");
}

#[test]
fn test_invalid_assignment() {
    let program = r#"
        let a = 42;
        42 = a;
    "#;
    execute_program_expect_error(program, "Expected \';\' after expression");
}

#[test]
fn test_invalid_variable_declaration() {
    let program = r#"
        let 123abc = 42;
        print_value(123abc);
    "#;
    execute_program_expect_error(program, "Expected identifier after \'let\'");
}

#[test]
fn test_invalid_function_declaration() {
    let program = r#"
        fn 123invalid() {
            print_value(42);
        }
    "#;
    execute_program_expect_error(program, "Expected function name");
}
