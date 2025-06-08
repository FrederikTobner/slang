use crate::test_utils::execute_program_expect_error;

#[test]
fn type_mismatch_in_function_argument() {
    let program = r#"
        fn expect_int(x: i32) {}
        
        expect_int("not an integer");
    "#;
    execute_program_expect_error(
        program,
        "[E2010]",
        "Type mismatch: function \'expect_int\' expects argument 1 to be i32, but got string",
    );
}

#[test]
fn wrong_parameter_count() {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        print_value(add(5));
    "#;
    execute_program_expect_error(
        program,
        "[E2009]",
        "Function \'add\' expects 2 arguments, but got 1",
    );
}

#[test]
fn wrong_parameter_types() {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        print_value(add("hello", 5));
    "#;
    execute_program_expect_error(
        program,
        "[E2010]",
        "Type mismatch: function \'add\' expects argument 1 to be i32, but got string\n",
    );
}

#[test]
fn return_type_mismatch() {
    let program = r#"
        fn get_number() -> i32 {
            return "not a number";
        }
        
        print_value(get_number());
    "#;
    execute_program_expect_error(
        program,
        "[E2012]",
        "Type mismatch: function returns i32 but got string",
    );
}

#[test]
fn undefined_function() {
    let program = r#"
        let result = undefined_function(5, 10);
        print_value(result);
    "#;

    execute_program_expect_error(program, "[E2014]", "Undefined function");
}

#[test]
fn integer_return_type() {
    let program = r#"
        fn get_number() -> int {
            return 42;
        }
        
        let result = get_number();
        print_value(result);
    "#;

    execute_program_expect_error(
        program,
        "[E1030]",
        "\'int\' is not a valid type specifier. Use \'i32\', \'i64\', \'u32\', or \'u64\' instead",
    );
}

#[test]
fn duplicate_function_definition() {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        fn add(a: i32, b: i32) -> i32 {
            return a - b;
        }
        
        print_value(add(5, 10));
    "#;

    execute_program_expect_error(
        program,
        "[E2003]",
        "Function \'add\' is already defined in the current scope.",
    );
}
