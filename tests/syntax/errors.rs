use crate::test_utils::execute_program_expect_error;

#[test]
fn test_missing_semicolon() {
    let program = r#"
        let a = 42
        print_value(a);
    "#;
    execute_program_expect_error(program, "Compilation failed: Expected \';\' after let statement\n");
}

#[test]
fn test_mismatched_brackets() {
    let program = r#"
        fn main() {
            let a = 42;
            print_value(a);
    "#;
    execute_program_expect_error(program, "Compilation failed: Expected \'}\' after function body\n");
}

#[test]
fn test_mismatched_parentheses() {
    let program = r#"
        fn main() {
            let a = 42;
            print_value(a;
        }
    "#;
    execute_program_expect_error(program, "Compilation failed: Expected \')\' after function arguments\n");
}

#[test]
fn test_invalid_assignment() {
    let program = r#"
        let a = 42;
        42 = a;
    "#;
    execute_program_expect_error(program, "Compilation failed: Expected \';\' after expression\n");
}

#[test]
fn test_invalid_variable_declaration() {
    let program = r#"
        let 123abc = 42;
        print_value(123abc);
    "#;
    execute_program_expect_error(program, "Compilation failed: Expected identifier after \'let\'\n");
}

#[test]
fn test_invalid_function_declaration() {
    let program = r#"
        fn 123invalid() {
            print_value(42);
        }
    "#;
    execute_program_expect_error(program, "Compilation failed: Expected function name\n");
}
