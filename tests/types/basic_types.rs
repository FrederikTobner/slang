use crate::test_utils::execute_program_and_assert;

#[test]
fn test_integer_types() {
    let program = r#"
        let a: i32 = 42;
        let b: i64 = 42;
        let c: u32 = 42;
        let d: u64 = 42;
        print_value(a);
        print_value(b);
        print_value(c);
        print_value(d);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn test_float_type() {
    let program = r#"
        let a: f64 = 42.5;
        print_value(a);
    "#;
    execute_program_and_assert(program, "42.5");
}

#[test]
fn test_string_type() {
    let program = r#"
        let greeting: string = "Hello, world!";
        print_value(greeting);
    "#;
    execute_program_and_assert(program, "Hello, world!");
}

#[test]
fn test_type_inference() {
    let program = r#"
        let a = 42; 
        let b = "Hello";
        print_value(a);
        print_value(b);
    "#;
    execute_program_and_assert(program, "Hello");
    execute_program_and_assert(program, "42");
}