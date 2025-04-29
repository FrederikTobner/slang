use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};

#[test]
fn test_greater_than_operator_with_integers() {
    let program = r#"
        let result1 = 10 > 5;
        let result2 = 5 > 10;
        let result3 = 5 > 5;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\nfalse\nfalse");
}

#[test]
fn test_greater_than_operator_with_floats() {
    let program = r#"
        let result1 = 10.5 > 5.5;
        let result2 = 5.5 > 10.5;
        let result3 = 5.5 > 5.5;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\nfalse\nfalse");
}

#[test]
fn test_less_than_operator_with_integers() {
    let program = r#"
        let result1 = 5 < 10;
        let result2 = 10 < 5;
        let result3 = 5 < 5;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\nfalse\nfalse");
}

#[test]
fn test_less_than_operator_with_floats() {
    let program = r#"
        let result1 = 5.5 < 10.5;
        let result2 = 10.5 < 5.5;
        let result3 = 5.5 < 5.5;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\nfalse\nfalse");
}

#[test]
fn test_greater_equal_operator_with_integers() {
    let program = r#"
        let result1 = 10 >= 5;
        let result2 = 5 >= 10;
        let result3 = 5 >= 5;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\nfalse\ntrue");
}

#[test]
fn test_greater_equal_operator_with_floats() {
    let program = r#"
        let result1 = 10.5 >= 5.5;
        let result2 = 5.5 >= 10.5;
        let result3 = 5.5 >= 5.5;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\nfalse\ntrue");
}

#[test]
fn test_less_equal_operator_with_integers() {
    let program = r#"
        let result1 = 5 <= 10;
        let result2 = 10 <= 5;
        let result3 = 5 <= 5;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\nfalse\ntrue");
}

#[test]
fn test_less_equal_operator_with_floats() {
    let program = r#"
        let result1 = 5.5 <= 10.5;
        let result2 = 10.5 <= 5.5;
        let result3 = 5.5 <= 5.5;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\nfalse\ntrue");
}

#[test]
fn test_equal_operator_with_integers() {
    let program = r#"
        let result1 = 5 == 5;
        let result2 = 5 == 10;
        
        print_value(result1);
        print_value(result2);
    "#;
    execute_program_and_assert(program, "true\nfalse");
}

#[test]
fn test_equal_operator_with_floats() {
    let program = r#"
        let result1 = 5.5 == 5.5;
        let result2 = 5.5 == 10.5;
        
        print_value(result1);
        print_value(result2);
    "#;
    execute_program_and_assert(program, "true\nfalse");
}

#[test]
fn test_equal_operator_with_booleans() {
    let program = r#"
        let result1 = true == true;
        let result2 = false == false;
        let result3 = true == false;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\ntrue\nfalse");
}

#[test]
fn test_equal_operator_with_strings() {
    let program = r#"
        let result1 = "hello" == "hello";
        let result2 = "hello" == "world";
        
        print_value(result1);
        print_value(result2);
    "#;
    execute_program_and_assert(program, "true\nfalse");
}

#[test]
fn test_not_equal_operator_with_integers() {
    let program = r#"
        let result1 = 5 != 10;
        let result2 = 5 != 5;
        
        print_value(result1);
        print_value(result2);
    "#;
    execute_program_and_assert(program, "true\nfalse");
}

#[test]
fn test_not_equal_operator_with_floats() {
    let program = r#"
        let result1 = 5.5 != 10.5;
        let result2 = 5.5 != 5.5;
        
        print_value(result1);
        print_value(result2);
    "#;
    execute_program_and_assert(program, "true\nfalse");
}

#[test]
fn test_not_equal_operator_with_booleans() {
    let program = r#"
        let result1 = true != false;
        let result2 = true != true;
        let result3 = false != false;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\nfalse\nfalse");
}

#[test]
fn test_not_equal_operator_with_strings() {
    let program = r#"
        let result1 = "hello" != "world";
        let result2 = "hello" != "hello";
        
        print_value(result1);
        print_value(result2);
    "#;
    execute_program_and_assert(program, "true\nfalse");
}

#[test]
fn test_relational_operators_with_different_numeric_types() {
    let program = r#"
        let i32_val: i32 = 10;
        let i64_val: i64 = 10;
        let u32_val: u32 = 10;
        let u64_val: u64 = 10;
        let f32_val: f32 = 10.0;
        let f64_val: f64 = 10.0;       

        let eq1 = i32_val == i32_val;
        let eq2 = i64_val == i64_val;
        let eq3 = f32_val == f32_val;
        let eq4 = f64_val == f64_val;
        
        let gt1 = i32_val > 5;
        let gt2 = i64_val > 5;
        let gt3 = u32_val > 5;
        let gt4 = u64_val > 5;
        let gt5 = f32_val > 5.0;
        let gt6 = f64_val > 5.0;

        print_value(eq1);
        print_value(eq2);
        print_value(eq3);
        print_value(eq4);
        
        print_value(gt1);
        print_value(gt2);
        print_value(gt3);
        print_value(gt4);
        print_value(gt5);
        print_value(gt6);
    "#;
    execute_program_and_assert(program, "true\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue");
}

#[test]
fn test_relational_operators_with_incompatible_types() {
    let program = r#"
        let result = 5 > "string";
        print_value(result);
    "#;
    execute_program_expect_error(program, "Cannot compare different types with '>'");
    
    let program = r#"
        let result = true < 5;
        print_value(result);
    "#;
    execute_program_expect_error(program, "Cannot compare different types with '<'");
    
    let program = r#"
        let result = "hello" == 5;
        print_value(result);
    "#;
    execute_program_expect_error(program, "Cannot compare different types with '=='");
}

#[test]
fn test_relational_operators_in_complex_expressions() {
    let program = r#"
        let a = 5;
        let b = 10;
        let c = 15;
        
        let result1 = a < b && b < c;
        let result2 = a < b || b > c;
        let result3 = !(a > b) && b != c;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;
    execute_program_and_assert(program, "true\ntrue\ntrue");
}

#[test]
fn test_relational_operators_precedence() {
    let program = r#"
        let result1 = 5 + 5 > 8;
        let result2 = 5 > 3 + 2;
        
        let result3 = 5 > 3 && 10 < 20;
        let result4 = 5 > 10 || 3 < 5;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
        print_value(result4);
    "#;
    execute_program_and_assert(program, "true\nfalse\ntrue\ntrue");
}

#[test]
fn test_edge_cases_with_relational_operators() {
    let program = r#"
        let result1 = 0 == 0;
        let result2 = 0 > 0;
        let result3 = 0 >= 0;
        
        let result4 = 1000000000 > 999999999;
        
        let neg5 = 0 - 5; 
        let neg3 = 0 - 3;
        let neg10 = 0 - 10;
        
        let result5 = neg5 < neg3;
        let result6 = neg5 > neg10;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
        print_value(result4);
        print_value(result5);
        print_value(result6);
    "#;
    execute_program_and_assert(program, "true\nfalse\ntrue\ntrue\ntrue\ntrue");
}