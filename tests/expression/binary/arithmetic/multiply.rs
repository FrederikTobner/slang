use crate::ErrorCode;
use crate::assertions::ProgramAssertion;
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn with_integer_variables(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 6;
        let b: {type_name} = 7;
        print_value(a * b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn with_float_variables(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 6.0;
        let b: {type_name} = 7.0;
        print_value(a * b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("42");
}

#[rstest]
#[case("")] // No type suffix
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn with_integer_literals(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        print_value(6{type_name} * 7{type_name});
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("42");
}

#[rstest]
#[case("")] // No type suffix
#[case("f32")]
#[case("f64")]
fn with_float_literals(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        print_value(6.0{type_name} * 7.0{type_name});
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("42");
}

#[test]
fn with_incompatible_types() {
    // Arrange
    // Define all the types we want to test
    let all_types = ["i32", "i64", "u32", "u64", "f32", "f64", "bool", "string"];
    // Valid combinations (types that can be added together)
    let valid_combinations = [
        ("i32", "i32"),
        ("i64", "i64"),
        ("u32", "u32"),
        ("u64", "u64"),
        ("f32", "f32"),
        ("f64", "f64"),
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
                _ => "20", // integers
            };

            let right_value = match right_type {
                "f32" | "f64" => "22.0",
                "string" => "\"world\"",
                "bool" => "false",
                _ => "22", // integers
            };

            let program = format!(
                r#"
                let a: {left_type} = {left_value};
                let b: {right_type} = {right_value};
                print_value(a * b);
                "#
            );

            let expected_error =
                format!("Type mismatch: cannot apply '*' operator on {left_type} and {right_type}");

            // Act & Assert
            ProgramAssertion::new(&program)
                .fails()
                .error_code(ErrorCode::OperationTypeMismatch)
                .stderr(&expected_error);
        }
    }
}

#[test]
fn with_unit() {
    // Arrange
    let program = r#"
        let x = ();
        let y = ();
        print_value(x * y);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::OperationTypeMismatch)
        .stderr("Type mismatch: cannot apply '*' operator on () and ()");
}

#[test]
fn with_function() {
    // Arrange
    let program = r#"
        fn my_function() {}
        print_value(my_function * my_function);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::OperationTypeMismatch)
        .stderr("Type mismatch: cannot apply '*' operator on fn() -> () and fn() -> ()");
}

#[test]
fn with_native_function() {
    // Arrange
    let program = r#"
        print_value * print_value;
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::OperationTypeMismatch)
        .stderr(
            "Type mismatch: cannot apply '*' operator on fn(unknown) -> i32 and fn(unknown) -> i32",
        );
}
