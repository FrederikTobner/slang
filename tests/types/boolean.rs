use crate::test_utils::execute_program_and_assert;

#[test]
fn test_boolean_assign() {
    interpret(
        r#"
        let b: bool = true;
        print_value(b);
        
        let c: bool = false;
        print_value(c);
        "#,
        &[InterpretResult::Output("true"), InterpretResult::Output("false")],
    );
}

#[test]
fn test_boolean_literal() {
    interpret(
        r#"
        print_value(true);
        print_value(false);
        "#,
        &[InterpretResult::Output("true"), InterpretResult::Output("false")],
    );
}

#[test]
fn test_boolean_reference() {
    interpret(
        r#"
        let a: bool = true;
        let b: bool = a;
        print_value(b);
        "#,
        &[InterpretResult::Output("true")],
    );
}