use crate::assertions::ProgramAssertion;

#[test]
fn ascii_character_recognition() {
    // Arrange
    let program = r#"
        let char_a = "a";
        let char_z = "z";
        let char_0 = "0";
        let char_9 = "9";
        print_value("characters recognized");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("characters recognized");
}

#[test]
fn special_character_recognition() {
    // Arrange
    let program = r#"
        let space = " ";
        let underscore = "_";
        print_value("special chars recognized");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("special chars recognized");
}

#[test]
fn unicode_character_recognition() {
    // Arrange
    let program = r#"
        let unicode_char = "ñ";
        let emoji_char = "🚀";
        print_value("unicode chars recognized");
    "#;

    // Act & Assert
    ProgramAssertion::new(program)
        .succeeds()
        .stdout("unicode chars recognized");
}
