use crate::ErrorCode;
use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("false")]
#[case("true")]
fn from_boolean_literal(#[case] value: &str) {
    let program = format!(
        r#"
        let boolean_var: bool = {};
        print_value(boolean_var);
    "#,
        value
    );
    execute_program_and_assert(&program, value);
}

#[test]
fn boolean_type_inference() {
    let program = r#"
        let is_true = true;
        print_value(is_true);
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn from_string_literal() {
    let program = r#"
        let a: bool = "Hello";
        print_value(a);
    "#;
    execute_program_expect_error(
        program,
        ErrorCode::TypeMismatch,
        "Type mismatch: variable a is bool but expression is string",
    );
}

#[rstest]
#[case("42", "int")] // Integer literal
#[case("42i32", "i32")] // I32 literal
#[case("42i64", "i64")] // I64 literal
#[case("42u32", "u32")] // U32 literal
#[case("42u64", "u64")] // U64 literal
fn from_integer_literal(#[case] value: &str, #[case] _type: &str) {
    let program = format!(
        r#"
        let a: bool = {};
        print_value(a);
    "#,
        value
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is bool but expression is {}",
            _type
        ),
    );
}

#[rstest]
#[case("3.14", "float")] // Float literal
#[case("3.14f32", "f32")] // F32 literal
#[case("3.14f64", "f64")] // F64 literal
fn from_float_literal(#[case] value: &str, #[case] _type: &str) {
    let program = format!(
        r#"
        let a: bool = {};
        print_value(a);
    "#,
        value
    );
    execute_program_expect_error(
        &program,
        ErrorCode::TypeMismatch,
        &format!(
            "Type mismatch: variable a is bool but expression is {}",
            _type
        ),
    );
}

#[test]
fn using_boolean_type_as_name() {
    let program = r#"
        let bool: bool = true;
    "#;
    execute_program_expect_error(
        &program,
        ErrorCode::SymbolRedefinition,
        "Symbol \'bool\' of kind \'variable (conflicts with type)\' is already defined or conflicts with an existing symbol.",
    );
}
