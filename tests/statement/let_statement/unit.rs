use crate::test_utils::execute_program_and_assert;

#[test]
fn unit_literal() {
    let program = r#"
        let x = ();
        print_value(x);
    "#;
    execute_program_and_assert(program, "()");
}

#[test]
fn unit_type_annotation() {
    let program = r#"
        let x: () = ();
        print_value(x);
    "#;
    execute_program_and_assert(program, "()");
}
