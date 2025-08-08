use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn type_mismatch_in_function_argument() {
    // Arrange
    let program = r#"
        fn expect_int(x: i32) {}
        
        expect_int("not an integer");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ArgumentTypeMismatch)
        .stderr(
            "Type mismatch: function \'expect_int\' expects argument 1 to be i32, but got string",
        );
}

#[test]
fn wrong_parameter_count() {
    // Arrange
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        print_value(add(5));
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ArgumentCountMismatch)
        .stderr("Function \'add\' expects 2 arguments, but got 1");
}

#[test]
fn wrong_parameter_types() {
    // Arrange
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        print_value(add("hello", 5));
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ArgumentTypeMismatch)
        .stderr("Type mismatch: function \'add\' expects argument 1 to be i32, but got string\n");
}

#[test]
fn return_type_mismatch() {
    // Arrange
    let program = r#"
        fn get_number() -> i32 {
            return "not a number";
        }
        
        print_value(get_number());
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::ReturnTypeMismatch)
        .stderr("Type mismatch: function returns i32 but got string");
}

#[test]
fn undefined_function() {
    // Arrange
    let program = r#"
        let result = undefined_function(5, 10);
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::UndefinedFunction)
        .stderr("Undefined function");
}

#[test]
fn integer_return_type() {
    // Arrange
    let program = r#"
        fn get_number() -> int {
            return 42;
        }
        
        let result = get_number();
        print_value(result);
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::UnknownType)
        .stderr("\'int\' is not a valid type specifier. Use \'i32\', \'i64\', \'u32\', or \'u64\' instead");
}

#[test]
fn duplicate_function_definition() {
    // Arrange
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        fn add(a: i32, b: i32) -> i32 {
            return a - b;
        }
        
        print_value(add(5, 10));
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::SymbolRedefinition)
        .stderr("Function \'add\' is already defined in the current scope.");
}
