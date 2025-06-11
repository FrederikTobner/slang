use crate::test_utils::execute_program_and_assert;

#[test]
fn without_return() {
    let program = r#"
        let result = {
            let x = 42;
            let y = x + 1;
        };
        print_value(result);
    "#;
    execute_program_and_assert(program, "()");
}
