use crate::test_utils::execute_program_and_assert;

#[test]
fn with_multiple_params() {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        print_value(add(20, 22));
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn function_with_no_params() {
    let program = r#"
        fn get_magic_number() -> i32 {
            return 42;
        }
        
        print_value(get_magic_number());
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_empty_return() {
    let program = r#"
        fn void_function() {
            return;
        }
        
        void_function();
        print_value(42); // Just to verify program continues
    "#;
    execute_program_and_assert(program, "42");
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

#[test]
fn nested_function_calls() {
    let program = r#"
        fn add(a: i32, b: i32)-> i32 {
            return a + b;
        }
        
        fn multiply(a: i32, b: i32) -> i32 {
            return a * b;
        }
        
        print_value(add(multiply(3, 10), 12));
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn arguments_are_passed_by_value() {
    let program = r#"
        fn modify_value(x: i32) -> i32 {
            x = x + 10;
            return x;
        }
        
        let mut a : i32 = 5;
        let b = modify_value(a);
        
        print_value(a); // Should print 5, not 15
    "#;
    execute_program_and_assert(program, "5");
}