use super::super::error::SemanticAnalysisError;
use slang_ir::Location;
use slang_shared::CompilationContext;
use slang_types::{TypeId};

/// Type alias for result of semantic analysis operations
pub type SemanticResult = Result<TypeId, SemanticAnalysisError>;

/// Helper functions for common type checking operations.
/// This module provides utility functions that are shared across different operation types.

/// Creates a boolean type ID.
/// This is a convenience function used by relational and logical operations.
///
/// ### Returns
/// * `TypeId` representing the boolean type
pub fn bool_type() -> TypeId {
    TypeId::bool()
}

/// Creates an operation type mismatch error.
/// This is a convenience function to create consistent error messages across operations.
///
/// ### Arguments
/// * `operator` - The operator that failed
/// * `left_type` - The type of the left operand
/// * `right_type` - The type of the right operand
/// * `location` - The source location of the operation
///
/// ### Returns
/// * `SemanticAnalysisError` with operation type mismatch details
pub fn operation_type_mismatch_error(
    operator: &str,
    left_type: &TypeId,
    right_type: &TypeId,
    location: &Location,
) -> SemanticAnalysisError {
    SemanticAnalysisError::OperationTypeMismatch {
        operator: operator.to_string(),
        left_type: left_type.clone(),
        right_type: right_type.clone(),
        location: *location,
    }
}

/// Creates a logical operator type mismatch error.
/// This is a convenience function for logical operations that require boolean operands.
///
/// ### Arguments
/// * `operator` - The logical operator that failed
/// * `left_type` - The type of the left operand
/// * `right_type` - The type of the right operand
/// * `location` - The source location of the operation
///
/// ### Returns
/// * `SemanticAnalysisError` with logical operator type mismatch details
pub fn logical_operator_type_mismatch_error(
    operator: &str,
    left_type: &TypeId,
    right_type: &TypeId,
    location: &Location,
) -> SemanticAnalysisError {
    SemanticAnalysisError::LogicalOperatorTypeMismatch {
        operator: operator.to_string(),
        left_type: left_type.clone(),
        right_type: right_type.clone(),
        location: *location,
    }
}

/// Checks if two types are identical.
/// This is used by operations that require exact type matches.
///
/// ### Arguments
/// * `left_type` - The first type to compare
/// * `right_type` - The second type to compare
///
/// ### Returns
/// * `true` if the types are identical
/// * `false` otherwise
pub fn types_are_identical(left_type: &TypeId, right_type: &TypeId) -> bool {
    left_type == right_type
}

/// Checks if a type is a numeric type (integer or float).
/// This is used to validate operations that only work on numeric types.
///
/// ### Arguments
/// * `_context` - The compilation context (unused but kept for API consistency)
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is numeric
/// * `false` otherwise
pub fn is_numeric_type(context: &CompilationContext, type_id: &TypeId) -> bool {
    if let Some(primitive) = context.get_primitive_type_from_id(type_id) {
        primitive.is_numeric()
    } else {
        false
    }
}

/// Checks if a type is a boolean type.
/// This is used by logical operations.
///
/// ### Arguments
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is boolean
/// * `false` otherwise
pub fn is_boolean_type(type_id: &TypeId) -> bool {
    *type_id == TypeId::bool()
}

/// Checks if a type is the unit type.
/// Unit types are not allowed in most operations.
///
/// ### Arguments
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is unit
/// * `false` otherwise
pub fn is_unit_type(type_id: &TypeId) -> bool {
    *type_id == TypeId::unit()
}

/// Checks if a type is a string type.
/// String types have special rules for the addition operator (concatenation).
///
/// ### Arguments
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is string
/// * `false` otherwise
pub fn is_string_type(type_id: &TypeId) -> bool {
    *type_id == TypeId::string()
}

/// Checks if a type is an unspecified integer literal.
/// These can be coerced to specific integer types.
///
/// ### Arguments
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is an unspecified integer
/// * `false` otherwise
pub fn is_unspecified_integer_type(type_id: &TypeId) -> bool {
    *type_id == TypeId::unspecified_int()
}

/// Checks if a type is an unspecified float literal.
/// These can be coerced to specific float types.
///
/// ### Arguments
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is an unspecified float
/// * `false` otherwise
pub fn is_unspecified_float_type(type_id: &TypeId) -> bool {
    *type_id == TypeId::unspecified_float()
}
