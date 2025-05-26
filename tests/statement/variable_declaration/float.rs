use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("")]
#[case(": f32")]
#[case(": f64")]
fn from_literal(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a{} = 42.0;
        print_value(a);
    "#, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_literal_with_type_suffix(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a = 42.0{};
        print_value(a);
    "#, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("")] // No type suffix
#[case("f32")]
#[case("f64")]
fn from_binary_expression (
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a = 20.0{} + 22.0{};
        print_value(a);
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_true_literal(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = true;
    "#, type_name);
    execute_program_expect_error(&program, "[E2005]", &format!("Type mismatch: variable a is {} but expression is bool", type_name));
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_false_literal(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = false;
    "#, type_name);
    execute_program_expect_error(&program, "[E2005]", &format!("Type mismatch: variable a is {} but expression is bool", type_name));
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_string_literal(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = "hello";
    "#, type_name);
    execute_program_expect_error(&program, "[E2005]", &format!("Type mismatch: variable a is {} but expression is string", type_name));
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_integer_literal(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = 42;
    "#, type_name);
    execute_program_expect_error(&program, "[E2005]", &format!("Type mismatch: variable a is {} but expression is int", type_name));
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_integer_literal_with_i32_suffix(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = 42i32;
    "#, type_name);
    execute_program_expect_error(&program, "[E2005]", &format!("Type mismatch: variable a is {} but expression is i32", type_name));
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn from_integer_literal_with_i64_suffix(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = 42i64;
    "#, type_name);
    execute_program_expect_error(&program, "[E2005]", &format!("Type mismatch: variable a is {} but expression is i64", type_name));
}


#[rstest]
#[case("f32")]
#[case("f64")]
fn from_integer_literal_with_u32_suffix(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = 42u32;
    "#, type_name);
    execute_program_expect_error(&program, "[E2005]", &format!("Type mismatch: variable a is {} but expression is u32", type_name));
}



#[rstest]
#[case("f32")]
#[case("f64")]
fn from_float_literal_with_u64_suffix(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let a: {} = 42u64;
    "#, type_name);
    execute_program_expect_error(&program, "[E2005]", &format!("Type mismatch: variable a is {} but expression is u64", type_name));
}


#[test]
fn float_type() {
    let program = r#"
        let a: float = 0.0; 
    "#;
    execute_program_expect_error(program, "[E1029]", "\'float\' is not a valid type specifier. Use \'f32\' or \'f64\' instead");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn using_type_as_variable_name(
    #[case] type_name: &str,
) {
    let program = format!(r#"
        let {} = 42.0;
    "#, type_name);
    execute_program_expect_error(&program, "[E2003]", &format!("Symbol \'{}\' of kind \'variable (conflicts with type)\' is already defined or conflicts with an existing symbol.", type_name));
}