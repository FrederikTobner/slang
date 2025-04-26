use crate::test_utils::execute_program_and_assert;

#[test]
fn parentheses() {
    let program = r#"
        let a = (1 + 2) * 3;
        print_value(a);
    "#;
    execute_program_and_assert(program, "9\n");
}

#[test]
fn precedence() {
    let program = r#"
        let a = 1 + 2 * 3;
        print_value(a);
    "#;
    execute_program_and_assert(program, "7\n");
}