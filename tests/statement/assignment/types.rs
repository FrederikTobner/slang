use crate::test_utils::execute_program_and_assert;

#[test]
fn unit_assignment() {
    let program = r#"
        let mut x = ();
        x = ();
        print_value(x);
    "#;
    execute_program_and_assert(program, "()");
}
