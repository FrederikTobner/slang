use crate::assertions::ProgramAssertion;

#[test]
fn parentheses() {
    // Arrange
    let program = r#"
        let a = 3 * (1 + 2);
        print_value(a);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("9");
}

#[test]
fn precedence() {
    // Arrange
    let program = r#"
        let a = 1 + 2 * 3;
        print_value(a);
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("7");
}

#[test]
fn nested_blocks() {
    // Arrange
    let program = r#"
        let x = 10;
        {
            let y = 20;
            {
                let z = 30;
                print_value(x + y + z);
            }
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("60");
}

#[test]
fn variable_shadowing() {
    // Arrange
    let program = r#"
        let x = 5;
        {
            let x = 10; // Shadowing the outer x
            print_value(x); // Should print 10
        }
        print_value(x); // Should print 5
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("10\n5");
}

#[test]
fn variable_shadowing_with_different_types() {
    // Arrange
    let program = r#"
        let x = 5;
        {
            let x: string = "foo"; // Shadowing with a different type
            print_value(x); // Should print foo
        }
        print_value(x); // Should print 5
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("foo\n5");
}

#[test]
fn variable_in_different_scopes() {
    // Arrange
    let program = r#"
        {
            let y = 2;
            print_value(y); // Should print 3
        }
        {
            let y = 3; // This y is different from the previous one
            print_value(y); // Should print 3
        }
    "#;

    // Act & Assert
    ProgramAssertion::new(program).succeeds().stdout("2\n3");
}

#[test]
fn variable_in_nested_scopes_with_different_types() {
    // Arrange
    let program = r#"
        let x = 5;
        {
            let x: string = "foo"; // Shadowing with a different type
            print_value(x); // Should print foo
        }
        {
            let x = 10; // This x is different from the previous one
            print_value(x); // Should print 10
        }
        print_value(x); // Should print 5
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("foo\n10\n5");
}
