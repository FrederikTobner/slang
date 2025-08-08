use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;
use rstest::rstest;

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn equal_integer(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 5;
        let b: {type_name} = 5;
        
        print_value(a != b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("false");
}

#[rstest]
#[case("i32")]
#[case("i64")]
#[case("u32")]
#[case("u64")]
fn not_equal_integer(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 5;
        let b: {type_name} = 10;
        
        print_value(a != b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("true");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn equal_float(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 5.5;
        let b: {type_name} = 5.5;
        
        print_value(a != b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("false");
}

#[rstest]
#[case("f32")]
#[case("f64")]
fn not_equal_float(#[case] type_name: &str) {
    // Arrange
    let program = format!(
        r#"
        let a: {type_name} = 5.5;
        let b: {type_name} = 10.5;
        
        print_value(a != b);
    "#
    );

    // Act & Assert
    ProgramAssertion::new(&program).succeeds().stdout("true");
}

#[test]
fn with_booleans() {
    // Arrange
    let program = r#"
        let result1 = true != true;
        let result2 = false != false;
        let result3 = true != false;
        
        print_value(result1);
        print_value(result2);
        print_value(result3);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("false\nfalse\ntrue");
}

#[test]
fn with_strings() {
    // Arrange
    let program = r#"
        let result1 = "hello" != "hello";
        let result2 = "hello" != "world";
        
        print_value(result1);
        print_value(result2);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("false\ntrue");
}

#[test]
fn with_unit() {
    // Arrange
    let program = r#"
        let x = ();
        let y = ();
        print_value(x != y);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::OperationTypeMismatch)
        .stderr("Type mismatch: cannot apply '!=' operator on () and ()");
}

#[test]
fn with_function() {
    // Arrange
    let program = r#"
        fn my_function() {}
        let fun_1 = my_function;
        let fun_2 = my_function;
        print_value(fun_1 != fun_2);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("false");
}

#[test]
fn with_native_function() {
    // Arrange
    let program = r#"
        print_value(print_value != print_value);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("false");
}
