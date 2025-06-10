use crate::test_utils::execute_program_and_assert;

#[test]
fn simple_variable_reference() {
    let program = r#"
        let x = 42;
        print_value(x);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn string_variable_reference() {
    let program = r#"
        let message = "hello";
        print_value(message);
    "#;
    execute_program_and_assert(program, "hello");
}

#[test]
fn boolean_variable_reference() {
    let program = r#"
        let flag = true;
        print_value(flag);
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn unit_variable_reference() {
    let program = r#"
        let unit_value = ();
        print_value(unit_value);
    "#;
    execute_program_and_assert(program, "()");
}

#[test]
fn function_variable_reference() {
    let program = r#"
        fn greet() -> string {
            return "hello";
        }
        
        let greeting = greet;
        print_value(greeting);
    "#;
    execute_program_and_assert(program, "<fn greet>");
}

#[test]
fn native_function_reference() {
    let program = r#"
        let native_print = print_value;
        native_print(native_print);
    "#;
    execute_program_and_assert(program, "<native fn print_value>");
}