use crate::test_utils::execute_program_and_assert;

#[test]
fn basic() {
    let program = r#"
        print_value(());
    "#;
    execute_program_and_assert(program, "()");
}

