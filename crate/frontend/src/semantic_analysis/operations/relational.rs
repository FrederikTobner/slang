
use super::super::error::SemanticAnalysisError;
use super::helpers::{bool_type, operation_type_mismatch_error, types_are_identical, is_unspecified_integer_type, is_unspecified_float_type};
use slang_ir::ast::BinaryOperator;
use slang_ir::Location;
use slang_shared::CompilationContext;
use slang_types::{TypeId};

use super::super::type_system;

/// Type alias for result of semantic analysis operations
pub type SemanticResult = Result<TypeId, SemanticAnalysisError>;

/// Checks if an operator is a relational operator (requires numeric types)
/// 
/// ### Arguments
/// * `operator` - The binary operator to check
/// 
/// ### Returns
/// * `true` if the operator requires numeric types, `false` otherwise
fn is_strictly_relational_operator(operator: &BinaryOperator) -> bool {
    matches!(
        operator,
        BinaryOperator::GreaterThan
            | BinaryOperator::LessThan
            | BinaryOperator::GreaterThanOrEqual
            | BinaryOperator::LessThanOrEqual
    )
}

/// Checks if types are compatible for unspecified literal coercion in relational operations
/// 
/// ### Arguments
/// * `context` - The compilation context
/// * `left_type` - The type of the left operand
/// * `right_type` - The type of the right operand
/// 
/// ### Returns
/// * `true` if one operand is an unspecified literal compatible with the other, `false` otherwise
fn can_coerce_for_relational(
    context: &CompilationContext,
    left_type: &TypeId,
    right_type: &TypeId,
) -> bool {
    (is_unspecified_integer_type(left_type) && type_system::is_integer_type(context, right_type))
        || (is_unspecified_integer_type(right_type) && type_system::is_integer_type(context, left_type))
        || (is_unspecified_float_type(left_type) && type_system::is_float_type(context, right_type))
        || (is_unspecified_float_type(right_type) && type_system::is_float_type(context, left_type))
}

/// Checks if types are compatible for relational operations (>, <, >=, <=, ==, !=).
/// Types must be comparable with each other, which means they're either:
/// - Exactly the same type (except Unit)
/// - Unspecified integer literal and an integer type
/// - Unspecified float literal and a float type
///
/// ### Arguments
/// * `context` - The compilation context
/// * `left_type` - The type of the left operand
/// * `right_type` - The type of the right operand
/// * `operator` - The relational operator
/// * `location` - The source location of the operation
///
/// ### Returns
/// * `Ok(bool_type())` if the types are comparable
/// * `Err` with a descriptive error message otherwise
pub fn check_relational_operation(
    context: &CompilationContext,
    left_type: &TypeId,
    right_type: &TypeId,
    operator: &BinaryOperator,
    location: &Location,
) -> SemanticResult {
    // Strictly relational operators (>, <, >=, <=) require numeric types
    if is_strictly_relational_operator(operator)
        && (!context.is_numeric_type(left_type)
            || !context.is_numeric_type(right_type))
    {
        return Err(operation_type_mismatch_error(
            &operator.to_string(),
            left_type,
            right_type,
            location,
        ));
    }

    // Check for type compatibility
    if (types_are_identical(left_type, right_type) && *left_type != TypeId::unit())
        || can_coerce_for_relational(context, left_type, right_type)
    {
        Ok(bool_type())
    } else {
        Err(operation_type_mismatch_error(
            &operator.to_string(),
            left_type,
            right_type,
            location,
        ))
    }
}
