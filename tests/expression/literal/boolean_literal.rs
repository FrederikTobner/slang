use crate::test_utils::execute_program_and_assert;

#[test]
fn true_literal() {
    let program = r#"
        print_value(true);
    "#;
    execute_program_and_assert(program, "true");
}

#[test]
fn false_literal() {
    let program = r#"
        print_value(false);
    "#;
    execute_program_and_assert(program, "false");
}