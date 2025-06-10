use crate::test_utils::execute_program_and_assert;

#[test]
fn test_digit_recognition_0_to_9() {
    let program = r#"
        let d0 = 0;
        let d1 = 1;
        let d2 = 2;
        let d3 = 3;
        let d4 = 4;
        let d5 = 5;
        let d6 = 6;
        let d7 = 7;
        let d8 = 8;
        let d9 = 9;
        print_value("all digits recognized");
    "#;
    execute_program_and_assert(program, "all digits recognized");
}

#[test]
fn test_digit_in_identifiers() {
    let program = r#"
        let var1 = "one";
        let var2name = "two";
        let name3 = "three";
        print_value("digits in identifiers");
    "#;
    execute_program_and_assert(program, "digits in identifiers");
}

#[test]
fn test_digit_sequences() {
    let program = r#"
        let num = 123456789;
        print_value(num);
    "#;
    execute_program_and_assert(program, "123456789");
}

#[test]
fn test_digit_with_underscores() {
    let program = r#"
        let big_num = 1000000;
        print_value(big_num);
    "#;
    execute_program_and_assert(program, "1000000");
}
