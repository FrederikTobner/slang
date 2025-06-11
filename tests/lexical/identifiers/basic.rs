use crate::ErrorCode;
use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};

#[test]
fn valid_identifier() {
    let program = r#"
        let valid_name = 42;
        print_value(valid_name);
    "#;
    execute_program_and_assert(program, "42");
}

#[test]
fn identifier_with_underscore() {
    let program = r#"
        let private_var = "hidden";
        let my_var_name = "visible";
        print_value(private_var);
        print_value(my_var_name);
    "#;
    execute_program_and_assert(program, "hidden\nvisible");
}

#[test]
fn identifier_with_numbers() {
    let program = r#"
        let var1 = 10;
        let var2name = 20;
        let name3var = 30;
        print_value(var1);
        print_value(var2name);
        print_value(name3var);
    "#;
    execute_program_and_assert(program, "10\n20\n30");
}

#[test]
fn starting_with_number_error() {
    let program = r#"
        let 1invalid = 42;
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ExpectedIdentifier,
        "Expected identifier",
    );
}

#[test]
fn with_special_characters_error() {
    let program = r#"
        let invalid-name = 42;
    "#;
    execute_program_expect_error(program, ErrorCode::ExpectedEquals, "Expected '='");
}
