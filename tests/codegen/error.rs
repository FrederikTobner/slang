use crate::ErrorCode;
use crate::test_utils::ProgramAssertion;

#[test]
fn too_many_constants() {
    // Arrange
    let mut program = String::new();
    for i in 0..300 {
        program.push_str(&format!("print_value({i});\n"));
    }
    
    // Act & Assert
    ProgramAssertion::new(&program)
        .fails()
        .error_code(ErrorCode::TooManyConstants)
        .stderr("Too many constants");
}
