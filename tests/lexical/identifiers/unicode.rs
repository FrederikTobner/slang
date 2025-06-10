use crate::test_utils::execute_program_and_assert;

#[test]
fn basic() {
    let program = r#"
        let héllo = "unicode";
        print_value(héllo);
    "#;
    execute_program_and_assert(program, "unicode");
}

#[test]
fn emoji_identifier() {
    let program = r#"
        let rocket = "launch";
        print_value(rocket);
    "#;
    execute_program_and_assert(program, "launch");
}

#[test]
fn greek_identifier() {
    let program = r#"
        let π = 3.14159;
        print_value(π);
    "#;
    execute_program_and_assert(program, "3.14159");
}

#[test]
fn mixed_unicode_ascii() {
    let program = r#"
        let user_名前 = "name";
        print_value(user_名前);
    "#;
    execute_program_and_assert(program, "name");
}
