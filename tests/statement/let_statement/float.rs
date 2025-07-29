use crate::ErrorCode;
use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("")] // Type is inferred
#[case(": f32")]
#[case(": f64")]
fn from_literal(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a{type_name} = 42.0;
        print_value(a);
    "#
    );
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_literal_with_type_suffix(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a = 42.0{type_name};
        print_value(a);
    "#
    );
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("")] // No type suffix
#[case("f32")]
#[case("f64")]
fn from_binary_expression(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a = 20.0{type_name} + 22.0{type_name};
        print_value(a);
    "#
    );
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_true_literal(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = true;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is bool"
        ),
    );
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_false_literal(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = false;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is bool"
        ),
    );
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_string_literal(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = "hello";
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is string"
        ),
    );
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_integer_literal(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = 42;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is int"
        ),
    );
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_integer_literal_with_i32_suffix(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = 42i32;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is i32"
        ),
    );
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_integer_literal_with_i64_suffix(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = 42i64;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is i64"
        ),
    );
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_integer_literal_with_u32_suffix(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = 42u32;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is u32"
        ),
    );
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_float_literal_with_u64_suffix(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = 42u64;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is u64"
        ),
    );
}

#[test]
fn float_type() {
    let program = r#"
        let a: float = 0.0; 
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::UnknownType,
        "\'float\' is not a valid type specifier. Use \'f32\' or \'f64\' instead",
    );
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn using_type_as_variable_name(#[case] type_name: &str) {
    let program = format!(
        r#"
        let {type_name} = 42.0;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::SymbolRedefinition,
        &format!(
            "Symbol \'{type_name}\' of kind \'variable (conflicts with type)\' is already defined or conflicts with an existing symbol."
        ),
    );
}
