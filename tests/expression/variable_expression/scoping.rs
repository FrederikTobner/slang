use crate::ErrorCode;
use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};

#[test]
fn test_variable_scoping_in_blocks() {
    let program = r#"
        let x = 10;
        {
            let x = 20;
            print_value(x);
        }
        print_value(x);
    "#;
    execute_program_and_assert(program, "20\n10");
}

#[test]
fn test_variable_scoping_in_functions() {
    let program = r#"
        let global_var = 100;
        
        fn test_function() {
            let local_var = 200;
            print_value(global_var);
            print_value(local_var);
        }
        
        test_function();
        print_value(global_var);
    "#;
    execute_program_and_assert(program, "100\n200\n100");
}

#[test]
fn test_variable_shadowing() {
    let program = r#"
        let value = 1;
        print_value(value);
        
        {
            let value = 3;
            print_value(value);
        }
        
        print_value(value);
    "#;
    execute_program_and_assert(program, "1\n3\n1");
}

#[test]
fn test_variable_out_of_scope_error() {
    let program = r#"
        {
            let local_var = 42;
        }
        print_value(local_var); // Should be out of scope
    "#;
    execute_program_expect_error(program, ErrorCode::UndefinedVariable, "Undefined variable");
}
