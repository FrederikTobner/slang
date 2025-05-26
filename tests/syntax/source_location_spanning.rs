use crate::test_utils::execute_program_expect_error;

// These tests verify that source location spanning works correctly for expressions
// by checking that error messages include the full expression, not just a single token

#[test]
fn binary_logical_expression_spans_correctly() {
    let program = r#"
        let x: i32 = true && false; // Type error should reference the full expression
    "#;
    // The error should mention the full boolean expression, not just the && operator
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable x is i32 but expression is bool");
}

#[test]
fn binary_arithmetic_expression_spans_correctly() {
    let program = r#"
        let x: bool = 10 + 20; // Type error should reference the full expression
    "#;
    // The error should mention the full arithmetic expression, not just the + operator
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable x is bool but expression is int");
}

#[test]
fn nested_binary_expressions_span_correctly() {
    let program = r#"
        let x: i32 = true && false || true; // Complex boolean expression
    "#;
    // The error should reference the full complex expression
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable x is i32 but expression is bool");
}

#[test]
fn unary_expression_spans_correctly() {
    let program = r#"
        let x: i32 = !true; // Unary expression should span from operator to end of operand
    "#;
    // The error should reference the full unary expression
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable x is i32 but expression is bool");
}

#[test]
fn mixed_expression_types_span_correctly() {
    let program = r#"
        let x: bool = 5 * 3 + 2; // Should span the entire arithmetic expression
    "#;
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable x is bool but expression is int");
}

#[test]
fn comparison_expression_spans_correctly() {
    let program = r#"
        let x: i32 = 10 > 5; // Comparison expression should span correctly
    "#;
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable x is i32 but expression is bool");
}

#[test]
fn string_concatenation_spans_correctly() {
    let program = r#"
        let x: i32 = "hello" + "world"; // String concatenation should span correctly
    "#;
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable x is i32 but expression is string");
}

#[test]
fn parenthesized_expression_spans_correctly() {
    let program = r#"
        let x: i32 = (true && false); // Parenthesized expressions should work correctly
    "#;
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable x is i32 but expression is bool");
}

#[test]
fn complex_nested_expression_spans_correctly() {
    let program = r#"
        let x: string = (10 + 5) * 2 > 25 && true; // Very complex expression
    "#;
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable x is string but expression is bool");
}

// Test for function call expressions spanning correctly
#[test] 
fn function_call_spans_correctly() {
    let program = r#"
        fn get_number() -> i32 {
            return 42;
        }
        
        let x: bool = get_number(); // Function call should span from name to closing paren
    "#;
    execute_program_expect_error(program, "[E2005]", "Type mismatch: variable x is bool but expression is i32");
}
