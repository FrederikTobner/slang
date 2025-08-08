use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn basic_if_statement_true() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        if x > 3 {
            print_value("condition is true");
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("condition is true");
}

#[test]
fn basic_if_statement_false() {
    // Arrange
    let program = r#"
        let x: i32 = 2;
        if x > 3 {
            print_value("condition is true");
        }
        print_value("after if");
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("after if");
}

#[test]
fn if_else_statement_true() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        if x > 3 {
            print_value("true branch");
        } else {
            print_value("false branch");
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("true branch");
}

#[test]
fn if_else_statement_false() {
    // Arrange
    let program = r#"
        let x: i32 = 2;
        if x > 3 {
            print_value("true branch");
        } else {
            print_value("false branch");
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("false branch");
}

#[test]
fn if_statement_multiple_statements() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        if x > 3 {
            print_value("first");
            print_value("second");
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("first");
}

#[test]
fn if_else_multiple_statements() {
    // Arrange
    let program = r#"
        let x: i32 = 2;
        if x > 3 {
            print_value("true1");
            print_value("true2");
        } else {
            print_value("false1");
            print_value("false2");
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("false1");
}

#[test]
fn nested_if_statements() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 10;
        if x > 3 {
            if y > 8 {
                print_value("nested true");
            } else {
                print_value("nested false");
            }
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("nested true");
}

#[test]
fn if_with_non_boolean_condition() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        if x {
            print_value("should not work");
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch");
}

#[test]
fn if_with_string_condition() {
    // Arrange
    let program = r#"
        let x: string = "hello";
        if x {
            print_value("should not work");
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::TypeMismatch)
        .stderr("Type mismatch");
}

#[test]
fn if_statement_with_complex_condition() {
    // Arrange
    let program = r#"
        let x: i32 = 5;
        let y: i32 = 3;
        if x > y && x < 10 {
            print_value("complex condition works");
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("complex condition works");
}
