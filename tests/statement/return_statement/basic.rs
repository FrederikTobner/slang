use crate::test_utils::execute_program_and_assert;

#[test]
fn return_integer() {
    let program = r#"
        fn test_function() -> i32 {
            return 42;
        }
        
        let result = test_function();
        print_value(result);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn return_string() {
    let program = r#"
        fn test_function() -> string {
            return "hello";
        }
        
        let result = test_function();
        print_value(result);
    "#;
    execute_program_and_assert(program, "hello");
}

#[test]
fn return_boolean() {
    let program = r#"
        fn test_function() -> bool {
            return true;
        }
        
        let result = test_function();
        print_value(result);
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn return_float() {
    let program = r#"
        fn test_function() -> f64 {
            return 3.14;
        }
        
        let result = test_function();
        print_value(result);
    "#;
    execute_program_and_assert(program, "3.14");
}

#[test]
fn coerce_integer_return() {
    let program = r#"
        fn test_function() -> i32 {
            return 42 + 123;
        }
        
        let result: i32 = test_function();
        print_value(result);
    "#;
    execute_program_and_assert(program, "165");
}

#[test]
fn coerce_float_return() {
    let program = r#"
        fn test_function() -> f64 {
            return 3.14 + 2.86;
        }
        
        let result: f64 = test_function();
        print_value(result);
    "#;
    execute_program_and_assert(program, "6");
}