
use super::super::error::SemanticAnalysisError;
use super::helpers;
use slang_ir::ast::BinaryOperator;
use slang_ir::Location;
use slang_types::TypeId;

/// Type alias for result of semantic analysis operations
pub type SemanticResult = Result<TypeId, SemanticAnalysisError>;

/// Checks if types are compatible for logical operations (AND, OR).
/// Both operands must be boolean types.
///
/// ### Arguments
/// * `left_type` - The type of the left operand
/// * `right_type` - The type of the right operand
/// * `operator` - The logical operator (either And or Or)
/// * `location` - The source location of the operation
///
/// ### Returns
/// * `Ok(bool_type())` if both operands are boolean
/// * `Err` with a descriptive error message otherwise
pub fn check_logical_operation(
    left_type: &TypeId,
    right_type: &TypeId,
    operator: &BinaryOperator,
    location: &Location,
) -> SemanticResult {
    if helpers::is_boolean_type(left_type) && helpers::is_boolean_type(right_type) {
        Ok(helpers::bool_type())
    } else {
        Err(helpers::logical_operator_type_mismatch_error(
            &operator.to_string(),
            left_type,
            right_type,
            location,
        ))
    }
}
