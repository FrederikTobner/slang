use crate::test_utils::execute_program_expect_error;
use crate::ErrorCode;

#[test]
fn undefined_variable() {
    let program = r#"
        print_value(y); 
    "#;

    execute_program_expect_error(program, ErrorCode::UndefinedVariable, "Undefined variable: y");
}

#[test]
fn unknown_type() {
    let program = r#"
        let a: unknown = 0; 
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::UnknownType,
        "'unknown' is not a valid type specifier",
    );
}
