use crate::test_utils::execute_program_and_assert;

#[test]
fn early_return_in_if_statement() {
    let program = r#"
        fn test_function(x: i32) -> i32 {
            if x > 0 {
                return x * 2;
            }
            return 0;
        }
        
        let result1 = test_function(5);
        let result2 = test_function(-1);
        print_value(result1);
        print_value(result2);
    "#;
    execute_program_and_assert(program, "10\n0");
}

#[test]
fn multiple_return_paths() {
    let program = r#"
        fn test_function(x: i32) -> string {
            if x > 10 {
                return "large";
            } else {
                if x > 0 {
                    return "small";
                } else {
                    return "zero or negative";
                }
            }
        }
        
        let result1 = test_function(15);
        let result2 = test_function(5);
        let result3 = test_function(-1);
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "large\nsmall\nzero or negative");
}

#[test]
fn unreachable_code_after_return() {
    let program = r#"
        fn test_function() -> i64 {
            return 42;
            let x = 10; // This should be unreachable
            return x;
        }
        
        let result = test_function();
        print_value(result);
    "#;
    // This might be a warning rather than an error, depending on implementation
    execute_program_and_assert(program, "42");
}
