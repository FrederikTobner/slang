use crate::test_utils::execute_program_expect_error;
use crate::ErrorCode;

#[test]
fn type_mismatch() {
    let program = r#"
        fn add(x: i32, y: i32) -> i32 {
            return x + y;
        }
        
        let func_var: fn(string) -> i32 = add; // Type mismatch
    "#;
    execute_program_expect_error(program, ErrorCode::TypeMismatch, "Type mismatch");
}

#[test]
fn parameter_count_mismatch() {
    let program = r#"
        fn single_param(x: i32) -> i32 {
            return x;
        }
        
        let func_var: fn(i32, i32) -> i32 = single_param; // Parameter count mismatch
    "#;
    execute_program_expect_error(program, ErrorCode::TypeMismatch, "Type mismatch");
}

#[test]
fn return_mismatch() {
    let program = r#"
        fn returns_string() -> string {
            return "hello";
        }
        
        let func_var: fn() -> i32 = returns_string; // Return type mismatch
    "#;
    execute_program_expect_error(program, ErrorCode::TypeMismatch, "Type mismatch");
}
