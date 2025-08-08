use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn with_integer_types(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 50;
        let b: {type_name} = 8;
        print_value(if a > b {{ a - b }} else {{ b - a }});
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("42");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn with_float_types(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 50.0;
        let b: {type_name} = 8.0;
        print_value(if a > b {{ a - b }} else {{ b - a }});
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("42");
}

#[test]
fn with_strings() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let result: string = if x > 3 { "greater" } else { "lesser" };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("greater");
}

#[test]
fn with_booleans() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let result: bool = if x > 3 { true } else { false };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("true");
}

#[test]
fn nested() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 3;
        let result: i32 = if x > y { 
            if x > 10 { 100 } else { 50 } 
        } else { 
            if y > 10 { 200 } else { 25 } 
        };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("50");
}

#[test]
fn complex_nested() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 10;
        let z: i32 = 15;
        let result: i32 = if x < y {
            if y < z {
                let a: i32 = x + y;
                x + y + z + a
            } else {
                let b: i32 = y - x;
                y - x + b
            }
        } else {
            let c: i32 = z - y;
            z - y + c
        };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("45");
}

#[test]
fn block_with_multiple_statements() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let result: i32 = if x > 3 {
            let a: i32 = x + 2;
            let b: i32 = a * 2;
            b
        } else {
           x - 1 
        };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("14");
}

#[test]
fn in_function_call() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        print_value(if x > 3 { "true" } else { "false" });
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("true");
}

#[test]
fn with_complex_condition() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 3;
        let result: string = if x > y && x < 10 { "in range" } else { "out of range" };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("in range");
}

#[test]
fn type_mismatch() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let result: i32 = if x > 3 { 10 } else { "string" };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch");
}

#[test]
fn non_boolean_condition() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let result: i32 = if x { 10 } else { 20 };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch");
}

#[test]
fn with_arithmetic() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 3;
        let result: i32 = if x > y { x + y } else { x - y };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("8");
}

#[test]
fn chained() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let a: i32 = if x > 3 { 10 } else { 5 };
        let b: i32 = if a > 7 { 20 } else { 15 };
        print_value(b);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("20");
}

#[test]
fn must_have_same_type() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let result: i32 = if x > 3 { 10 } else { "string" };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch");
}

#[test]
fn must_have_else_branch() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let result: i32 = if x > 3 { 10 };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ExpectedElse)
        .stderr("Expected 'else' after if expression");
}

#[test]
fn with_unit_branches() {
    // Arrange
    let program = r#"
        let x = true;
        let result = if x { () } else { () };
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("()");
}

#[test]
fn with_function_branches() {
    // Arrange
    let program = r#"
        fn my_function() -> i32 {
            42
        }
        fn another_function() -> i32 {
            0
        }
        
        let x: bool = true;
        let result: fn() -> i32 = if x { my_function } else { another_function };
        print_value(result());
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("42");
}

#[test]
fn with_native_function_branches() {
    // Arrange
    let program = r#"
        let x: bool = true;
        let result = if x { print_value } else { print_value };
        print_value(result(100));
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("100");
}
