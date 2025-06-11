use crate::ErrorCode;
use crate::test_utils::execute_program_expect_error;

#[test]
fn too_many_constants() {
    let mut program = String::new();
    for i in 0..300 {
        program.push_str(&format!("print_value({});\n", i));
    }
    execute_program_expect_error(
        &program,
        ErrorCode::GenericCompileError,
        "Too many constants",
    );
}
