use crate::test_utils::execute_program_and_assert;

#[test]
fn complex_expression() {
    let program = r#"
        let a: bool = true;
        let b: bool = false;
        let c: bool = true;
        print_value(a && b || c); 
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn with_not() {
    let program = r#"
        let a: bool = true;
        let b: bool = true;
        print_value(!(a && b)); 
    "#;
    execute_program_and_assert(program, "false");
}

#[test]
fn precedence() {
    let program = r#"
        let a: bool = true;
        let b: bool = false;
        let c: bool = true; 
        let result = a && b || c;
        print_value(result);
    "#;
    execute_program_and_assert(program, "true");
}
