use crate::ErrorCode;
use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("")]
#[case(": i32")]
#[case(": i64")]
#[case(": u32")]
#[case(": u64")]
fn from_literal(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a{type_name} = 42;
        print_value(a);
    "#
    );
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_literal_with_type_suffix(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a = 42{type_name};
        print_value(a);
    "#
    );
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("")] // No type suffix
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_binary_expression(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a = 20{type_name} + 22{type_name};
        print_value(a);
    "#
    );
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
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
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
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
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
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
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_float_literal(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = 42.0;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is float"
        ),
    );
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_float_literal_with_f32_suffix(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = 42.0f32;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is f32"
        ),
    );
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn from_float_literal_with_f64_suffix(#[case] type_name: &str) {
    let program = format!(
        r#"
        let a: {type_name} = 42.0f64;
    "#
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is {type_name} but expression is f64"
        ),
    );
}

#[test]
fn int_type() {
    let program = r#"
        let a: int = 0; 
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::UnknownType,
        "\'int\' is not a valid type specifier. Use \'i32\', \'i64\', \'u32\', or \'u64\' instead",
    );
}

#[test]
fn i32_value_out_of_range() {
    let program = r#"
        let a: i32 = 2147483648; 
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ValueOutOfRange,
        "Integer literal 2147483648 is out of range for type i32",
    );
}

#[test]
fn u32_unsigned_negative_value_error() {
    let program = r#"
        let a: u32 = -1;
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::ValueOutOfRange,
        "Integer literal -1 is out of range for type u32",
    );
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
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
