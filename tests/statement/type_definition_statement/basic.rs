use crate::test_utils::execute_program_and_assert;

#[test]
fn simple_type() {
    let program = r#"
        struct MyStruct {
            field1: string,
            field2: i32,
        };
        print_value("struct defined successfully");
    "#;
    execute_program_and_assert(program, "struct defined successfully");
}

#[test]
fn empty_type() {
    let program = r#"
        struct EmptyStruct {};
        print_value("empty struct defined");
    "#;
    execute_program_and_assert(program, "empty struct defined");
}

#[test]
fn single_field_concept() {
    let program = r#"
        struct SingleFieldStruct {
            field: i32,
        };
        print_value("single field struct defined");
    "#;
    execute_program_and_assert(program, "single field struct defined");
}
