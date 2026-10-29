use crate::ErrorCode;
use crate::assertions::ProgramAssertion;
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn smaller_on_int(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 20;
        let b: {type_name} = 22;
        print_value(a <= b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("true");
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn equal_on_int(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 20;
        let b: {type_name} = 20;
        print_value(a <= b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("true");
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn greater_on_int(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 22;
        let b: {type_name} = 20;
        print_value(a <= b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("false");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn smaller_on_float(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 20.0;
        let b: {type_name} = 22.0;
        print_value(a <= b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("true");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn equal_on_float(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 20.0;
        let b: {type_name} = 20.0;
        print_value(a <= b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("true");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn greater_on_float(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 22.0;
        let b: {type_name} = 20.0;
        print_value(a <= b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("false");
}

#[test]
fn on_unit() {
    // Arrange
    let program = r#"
        let x = ();
        let y = ();
        print_value(x <= y);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::OperationTypeMismatch)
        .stderr("Type mismatch: cannot apply '<=' operator on () and ()");
}

#[test]
fn with_booleans() {
    // Arrange
    let program = r#"
        let result1 = true <= true;
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::OperationTypeMismatch)
        .stderr("Type mismatch: cannot apply '<=' operator on bool and bool");
}

#[test]
fn with_strings() {
    // Arrange
    let program = r#"
        let result1 = "hello" <= "hello";
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::OperationTypeMismatch)
        .stderr("Type mismatch: cannot apply '<=' operator on string and string");
}

#[test]
fn with_function() {
    // Arrange
    let program = r#"
        fn my_function() {}
        let fun_1 = my_function;
        let fun_2 = my_function;
        print_value(fun_1 <= fun_2);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::OperationTypeMismatch)
        .stderr("Type mismatch: cannot apply '<=' operator on fn() -> () and fn() -> ()");
}

#[test]
fn with_native_function() {
    // Arrange
    let program = r#"
        print_value(print_value <= print_value);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::OperationTypeMismatch)
        .stderr("Type mismatch: cannot apply '<=' operator on fn(unknown) -> i32 and fn(unknown) -> i32");
}
