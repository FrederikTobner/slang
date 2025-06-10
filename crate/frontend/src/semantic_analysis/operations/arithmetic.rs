
use crate::semantic_error::SemanticAnalysisError;
use slang_ir::ast::{BinaryExpr, BinaryOperator};
use slang_ir::Location;
use slang_shared::CompilationContext;
use slang_types::{PrimitiveType, TypeId};

use super::super::type_system;

/// Type alias for result of semantic analysis operations
pub type SemanticResult = Result<TypeId, SemanticAnalysisError>;

/// Checks if a type is compatible with an arithmetic operation when both operands have the same type.
/// Boolean types are not allowed for any arithmetic operation.
/// String types are only allowed for the Add operator (concatenation).
///
/// ### Arguments
/// * `context` - The compilation context
/// * `type_id` - The type of both operands
/// * `operator` - The arithmetic operator (+, -, *, /)
/// * `location` - The source location of the operation
///
/// ### Returns
/// * `Ok(type_id)` if the operation is allowed
/// * `Err` with a descriptive error message otherwise
pub fn check_same_type_arithmetic(
    context: &CompilationContext,
    type_id: &TypeId,
    operator: &BinaryOperator,
    location: &Location,
) -> SemanticResult {
    if *type_id == TypeId(PrimitiveType::Bool as usize)
        || *type_id == TypeId(PrimitiveType::Unit as usize)
        || (operator != &BinaryOperator::Add
            && *type_id == TypeId(PrimitiveType::String as usize))
        || context.is_function_type(type_id)
    {
        Err(SemanticAnalysisError::OperationTypeMismatch {
            operator: operator.to_string(),
            left_type: type_id.clone(),
            right_type: type_id.clone(),
            location: *location,
        })
    } else {
        Ok(type_id.clone())
    }
}

/// Checks if mixed-type arithmetic operations are allowed, particularly handling
/// unspecified literals that can be coerced to match the other operand's type.
/// Handles the following cases:
/// - Unspecified integer literal + specific integer type
/// - Unspecified float literal + specific float type
/// - String concatenation with the + operator
///
/// ### Arguments
/// * `context` - The compilation context
/// * `left_type` - The type of the left operand
/// * `right_type` - The type of the right operand
/// * `bin_expr` - The binary expression containing both operands and the operator
///
/// ### Returns
/// * `Ok(type_id)` with the resulting operation type if allowed
/// * `Err` with a descriptive error message if the operation is not allowed
pub fn check_mixed_arithmetic_operation(
    context: &CompilationContext,
    left_type: &TypeId,
    right_type: &TypeId,
    bin_expr: &BinaryExpr,
) -> SemanticResult {
    type_system::check_mixed_arithmetic_operation(context, left_type, right_type, bin_expr)
}
