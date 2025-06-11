use crate::test_utils::execute_program_and_assert;

#[test]
fn simple() {
    let program = r#"
        fn some_function(x: i32) -> i32 {
            return x * 2;
        }
        
        let result = some_function(21);
        print_value(result);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn no_parameters() {
    let program = r#"
        fn get_value() -> i32 {
            return 42;
        }
        
        let result = get_value();
        print_value(result);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn no_return() {
    let program = r#"
        fn print_number(x: i32) {
            print_value(x);
        }
        
        print_number(42);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn multiple_parameters() {
    let program = r#"
        fn complex_function(x: i32, y: string, z: bool) -> string {
            if z {
                return y;
            } else {
                return "default";
            }
        }
        
        let result = complex_function(42, "hello", true);
        print_value(result);
    "#;
    execute_program_and_assert(program, "hello");
}
