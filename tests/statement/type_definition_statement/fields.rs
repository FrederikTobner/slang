use crate::test_utils::execute_program_and_assert;

#[test]
fn function_type_field() {
    let program = r#"
        struct Callback {
            callback: fn(string) -> string,
        };
        print_value("struct with function type field defined");
    "#;
    execute_program_and_assert(program, "struct with function type field defined");
}
