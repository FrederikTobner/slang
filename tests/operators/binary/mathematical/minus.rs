use crate::test_utils::{execute_program_and_assert, execute_program_expect_error};
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn with_integer_types(#[case] type_name: &str) {
    let program = format!(r#"
        let a: {} = 50;
        let b: {} = 8;
        print_value(a - b);
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn with_float_types(#[case] type_name: &str) {
    let program = format!(r#"
        let a: {} = 50.0;
        let b: {} = 8.0;
        print_value(a - b);
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}
#[rstest]
#[case("")] // No type suffix
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn with_integer_literals(#[case] type_name: &str) {
    let program = format!(r#"
        print_value(50{} - 8{});
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}

#[rstest]
#[case("")] // No type suffix
#[case("f32")]
#[case("f64")]
fn with_float_literals(#[case] type_name: &str) {
    let program = format!(r#"
        print_value(50.0{} - 8.0{});
    "#, type_name, type_name);
    execute_program_and_assert(&program, "42");
}

#[test]
fn with_incompatible_types() {
    // Define all the types we want to test
    let all_types =  ["i32", "i64", "u32", "u64", "f32", "f64", "bool", "string"];    
    // Valid combinations (types that can be added together)
    let valid_combinations = [
        ("i32", "i32"), ("i64", "i64"), ("u32", "u32"), ("u64", "u64"), 
        ("f32", "f32"), ("f64", "f64") 
    ];
    
    for &left_type in &all_types {
        for &right_type in &all_types {
            // Skip if it's a valid combination
            if valid_combinations.contains(&(left_type, right_type)) {
                continue;
            }
            
            // Create appropriate test values based on type
            let left_value = match left_type {
                "f32" | "f64" => "20.0",
                "string" => "\"hello\"",
                "bool" => "true",
                _ => "20"  // integers
            };
            
            let right_value = match right_type {
                "f32" | "f64" => "22.0",
                "string" => "\"world\"",
                "bool" => "false",
                _ => "22"  // integers
            };
            
            let program = format!(
                r#"
                let a: {} = {};
                let b: {} = {};
                print_value(a - b);
                "#,
                left_type, left_value, right_type, right_value
            );
            
            let expected_error = format!(
                "Type mismatch: cannot apply '-' operator on {} and {}", 
                left_type, right_type
            );
            
            execute_program_expect_error(&program, &expected_error);
        }
    }
}

