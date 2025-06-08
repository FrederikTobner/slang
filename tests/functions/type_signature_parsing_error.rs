use crate::test_utils::{execute_program_expect_error};

#[test]
fn missing_opening_parenthesize() {
    let program = r#"
        let my_function2 : fn i32 -> () = my_function;
        "#;
    execute_program_expect_error(program, "[E1006]", " Expected \'(\' after \'fn\'");
}


#[test]
fn missing_closing_parentesize() {
    let program = r#"
        let my_function2 : fn(i32 -> = my_function;
        "#;
    execute_program_expect_error(program, "[E1003]", " Expected \')\' after function parameters");
}

#[test]
fn missing_type_identifier() {
    let program = r#"
        let my_function2 : fn(i32) -> = my_function;
        "#;
    execute_program_expect_error(program, "[E1007]", " Expected type identifier");
}
