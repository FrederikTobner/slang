use crate::test_utils::execute_program_and_assert;

#[test]
fn precedence() {
    let program = r#"
        let a: i32 = 1;
        let b: i32 = 2;
        let c: i32 = 3;
        let result = a + b * c;
        print_value(result);
    "#;
    execute_program_and_assert(program, "7");
}

