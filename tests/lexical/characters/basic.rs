use crate::test_utils::execute_program_and_assert;

#[test]
fn ascii_character_recognition() {
    let program = r#"
        let char_a = "a";
        let char_z = "z";
        let char_0 = "0";
        let char_9 = "9";
        print_value("characters recognized");
    "#;
    execute_program_and_assert(program, "characters recognized");
}

#[test]
fn special_character_recognition() {
    let program = r#"
        let space = " ";
        let underscore = "_";
        print_value("special chars recognized");
    "#;
    execute_program_and_assert(program, "special chars recognized");
}

#[test]
fn unicode_character_recognition() {
    let program = r#"
        let unicode_char = "ñ";
        let emoji_char = "🚀";
        print_value("unicode chars recognized");
    "#;
    execute_program_and_assert(program, "unicode chars recognized");
}
