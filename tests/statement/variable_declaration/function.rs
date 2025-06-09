use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use crate::ErrorCode;

#[test]
fn with_explicit_function_type_mismatch() {
    let program = r#"
        fn my_print(value: string) {
            print_value(value);
        }
        
        let my_function : fn(i32) -> () = my_print;
        "#;
        execute_program_expect_error(program, ErrorCode::TypeMismatch, "Type mismatch: variable my_function is fn(i32) -> () but expression is fn(string) -> ()");
}


#[test]
fn with_explicit_function_type() {
    let program = r#"
         fn my_print(value: string) {
            print_value(value);
        }
        let my_function : fn(string) -> () = my_print;
        my_function("Hello from native function");
        "#;
    execute_program_and_assert(program, "Hello from native function");
}

#[test]
fn assign_native_to_function_with_explicit_function_type_multiple_times() {
    let program = r#"
        fn my_print(value: string) {
            print_value(value);
        }
        
        let my_function : fn(string) -> () = my_print;
        let my_function2 : fn(string) -> () = my_print;
        let my_function3 : fn(string) -> () = my_print;
        my_function("Hello from native function");
        my_function2("Hello from native function");
        my_function3("Hello from native function");
        "#;
    execute_program_and_assert(program, "Hello from native function\nHello from native function\nHello from native function");
}


#[test]
fn with_explicit_unit_return_type() {
    let program = r#"
        fn return_unit() -> () {
            return ();
        }
        
        let result = return_unit();
        print_value(result); // Should print nothing or "()" depending on implementation
    "#;
    execute_program_and_assert(program, "()");
}

#[test]
fn with_different_return_types() {
    let program = r#"
        fn get_string() -> string {
            return "Hello world";
        }
        
        fn get_int() -> i32 {
            return 42;
        }
        
        fn get_float() -> f64 {
            return 42.5;
        }
        
        print_value(get_string());
        print_value(get_int());
        print_value(get_float());
    "#;
    execute_program_and_assert(program, "Hello world");
    execute_program_and_assert(program, "42");
    execute_program_and_assert(program, "42.5");
}
