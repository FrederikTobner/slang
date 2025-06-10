use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use crate::ErrorCode;

#[test]
fn returns_unit_implicitly() {
    let program = r#"
        fn test_function() -> () {
            return;
        }
        
        test_function();
        print_value("completed");
    "#;
    execute_program_and_assert(program, "completed");
}

#[test]
fn without_expression_in_non_unit_function_error() {
    let program = r#"
        fn test_function() -> i32 {
            return;
        }
    "#;
    execute_program_expect_error(program, ErrorCode::MissingReturnValue, "Type mismatch");
}
