use crate::test_utils::execute_program_expect_error;

#[test]
fn undefined_variable() {
    let program = r#"
        print_value(y); 
    "#;
    
    execute_program_expect_error(program, "Undefined variable: y");
}



#[test]
fn unknown_type() {
    let program = r#"
        let a: unknown = 0; 
    "#;
    execute_program_expect_error(program, "Unknown type: unknown");
}

