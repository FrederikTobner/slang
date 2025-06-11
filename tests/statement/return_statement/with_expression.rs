use crate::ErrorCode;
use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};

#[test]
fn with_arithmetic_expression() {
    let program = r#"
        fn test_function() -> i64 {
            return 20i64 + 22i64;
        }
        
        let result = test_function();
        print_value(result);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn with_variable_expression() {
    let program = r#"
        fn test_function() -> i64 {
            let x = 42;
            return x;
        }
        
        let result = test_function();
        print_value(result);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn with_function_call() {
    let program = r#"
        fn inner_function() -> i32 {
            return 42;
        }
        
        fn outer_function() -> i32 {
            return inner_function();
        }
        
        let result = outer_function();
        print_value(result);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn return_type_mismatch_error() {
    let program = r#"
        fn test_function() -> i32 {
            return "hello";
        }
    "#;
    execute_program_expect_error(program, ErrorCode::ReturnTypeMismatch, "Type mismatch");
}
