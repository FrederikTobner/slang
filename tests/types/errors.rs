use crate::test_utils::execute_program_expect_error;

#[test]
fn test_type_mismatch_assignment() {
    let program = r#"
        let x: i32 = "not an integer";
    "#;
    
    execute_program_expect_error(program, "Type mismatch: variable x is i32 but expression is string");
}

#[test]
fn test_incompatible_binary_operands() {
    let program = r#"
        let a: i32 = 5;
        let b: string = "hello";
        print_value(a + b); 
    "#;
    
    execute_program_expect_error(program, "Type mismatch: cannot perform Plus operation with i32 and string");
}

#[test]
fn test_undefined_variable() {
    let program = r#"
        let x: i32 = 10;
        print_value(y); 
    "#;
    
    execute_program_expect_error(program, "Undefined variable: y");
}

#[test]
fn test_incompatible_numeric_types() {
    let program = r#"
        let a: i32 = 42;
        let b: f64 = 3.14;
        let c = a + b;
    "#;
    execute_program_expect_error(program, "Type mismatch: cannot perform Plus operation with i32 and f64\n");
}



#[test]
fn test_invalid_operation_for_type() {
    let program = r#"
        let a = "string";
        let b = a * 3;
    "#;
    execute_program_expect_error(program, "Type mismatch: cannot perform Multiply operation with string and int\n");
}

#[test]
fn test_i32_value_out_of_range() {
    let program = r#"
        let a: i32 = 2147483648; 
    "#;
    execute_program_expect_error(program, "Integer literal 2147483648 is out of range for type i32");
}

#[test]
fn test_unsigned_negative_value_error() {
    let program = r#"
        let a: u32 = -1;
    "#;
    execute_program_expect_error(program, "Integer literal -1 is out of range for type u32");
}

#[test]
fn test_int_type() {
    let program = r#"
        let a: int = 0; 
    "#;
    execute_program_expect_error(program, "\'int\' is not a valid type specifier. Use \'i32\', \'i64\', \'u32\', or \'u64\' instead");
}

#[test]
fn test_float_type() {
    let program = r#"
        let a: float = 0.0; 
    "#;
    execute_program_expect_error(program, "\'float\' is not a valid type specifier. Use \'f32\' or \'f64\' instead");
}
