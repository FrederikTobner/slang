use crate::ErrorCode;
use crate::assertions::ProgramAssertion;

#[test]
fn mutable_with_type_annotation() {
    // Arrange
    let program = r#"
        let mut x: i32 = 42;
        x = 50;
        print_value(x);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("50");
}

#[test]
fn mutable_with_type_inference() {
    // Arrange
    let program = r#"
        let mut x = 42;
        x = 50;
        print_value(x);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("50");
}

#[test]
fn multiple_assignments_to_mutable() {
    // Arrange
    let program = r#"
        let mut x = 10;
        x = 20;
        x = 30;
        x = x + 5;
        print_value(x);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("35");
}

#[test]
fn mixed_mutable_immutable() {
    // Arrange
    let program = r#"
        let x = 10;      // immutable
        let mut y = 20;  // mutable
        y = y + x;       // OK: reading from immutable, writing to mutable
        print_value(y);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("30");
}

#[test]
fn block_scope_mutability() {
    // Arrange
    let program = r#"
        let mut x = 10;
        {
            x = 20; // Should work, x is mutable in outer scope
            print_value(x);
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("20");
}

#[test]
fn with_immutable_variable() {
    // Arrange
    let program = r#"
        let x: i32 = 10;
        x = 20; // This should cause an error
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::AssignmentToImmutableVariable)
        .stderr("Cannot assign to immutable variable 'x'");
}

#[test]
fn with_immutable_in_expression() {
    // Arrange
    let program = r#"
        let x = 10;
        let y = 20;
        x = y + 5; // Should fail
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .fails()
        .error_code(ErrorCode::AssignmentToImmutableVariable)
        .stderr("Cannot assign to immutable variable 'x'");
}
