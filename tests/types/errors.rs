use crate::test_utils::execute_program_expect_error;


#[test]
fn test_type_mismatch_assignment() {
    let program = r#"
        let x: i32 = "not an integer";
    "#;
    
    execute_program_expect_error(program, "Compilation failed: Type mismatch: variable x is i32 but expression is string\n");
}

#[test]
fn test_incompatible_binary_operands() {
    let program = r#"
        let a: i32 = 5;
        let b: string = "hello";
        print_value(a + b); 
    "#;
    
    execute_program_expect_error(program, "Compilation failed: Type mismatch: cannot perform Plus operation with i32 and string\n");
}

#[test]
fn test_undefined_variable() {
    let program = r#"
        let x: i32 = 10;
        print_value(y); 
    "#;
    
    execute_program_expect_error(program, "Undefined variable");
}

#[test]
fn test_type_mismatch_in_operation() {
    let program = r#"
        let a = 42;
        let b = "string";
        let c = a + b;
    "#;
    execute_program_expect_error(program, "type");
}

#[test]
fn test_incompatible_numeric_types() {
    let program = r#"
        let a: i32 = 42;
        let b: f64 = 3.14;
        let c = a + b;
    "#;
    execute_program_expect_error(program, "Compilation failed: Type mismatch: cannot perform Plus operation with i32 and f64\n");
}



#[test]
fn test_invalid_operation_for_type() {
    let program = r#"
        let a = "string";
        let b = a * 3;
    "#;
    execute_program_expect_error(program, "Compilation failed: Integer literal 3 is out of range for type string\n");
}

#[test]
fn test_value_out_of_range() {
    let program = r#"
        let a: i32 = 2147483648; 
    "#;
    execute_program_expect_error(program, "range");
}

#[test]
fn test_integer_range_validation() {
    // Test that a value out of range for i32 is rejected
    let program = r#"
        let a: i32 = 2147483648;
        print_value(a);
    "#;
    execute_program_expect_error(program, "out of range");
}

#[test]
fn test_unsigned_negative_value_error() {
    let program = r#"
        let a: u32 = -1;
        print_value(a);
    "#;
    execute_program_expect_error(program, "Compilation failed: Type mismatch: variable a is u32 but expression is int\n");
}