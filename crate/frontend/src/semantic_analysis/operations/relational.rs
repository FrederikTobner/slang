
use super::super::error::SemanticAnalysisError;
use slang_ir::ast::BinaryOperator;
use slang_ir::Location;
use slang_shared::CompilationContext;
use slang_types::{PrimitiveType, TypeId};

use super::super::type_system;

/// Type alias for result of semantic analysis operations
pub type SemanticResult = Result<TypeId, SemanticAnalysisError>;

/// Checks if types are compatible for relational operations (>, <, >=, <=, ==, !=).
/// Types must be comparable with each other, which means they're either:
/// - Exactly the same type
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
    let is_relational = matches!(
        operator,
        BinaryOperator::GreaterThan
            | BinaryOperator::LessThan
            | BinaryOperator::GreaterThanOrEqual
            | BinaryOperator::LessThanOrEqual
    );

    if is_relational
        && (!PrimitiveType::from_int(left_type.0).is_some_and(|f| f.is_numeric())
            || !PrimitiveType::from_int(right_type.0).is_some_and(|f| f.is_numeric()))
    {
        return Err(SemanticAnalysisError::OperationTypeMismatch {
            operator: operator.to_string(),
            left_type: left_type.clone(),
            right_type: right_type.clone(),
            location: *location,
        });
    }

    if (left_type == right_type && *left_type != TypeId(PrimitiveType::Unit as usize))
        || (*left_type == TypeId(PrimitiveType::UnspecifiedInt as usize)
            && type_system::is_integer_type(context, right_type))
        || (*right_type == TypeId(PrimitiveType::UnspecifiedInt as usize)
            && type_system::is_integer_type(context, left_type))
        || (*left_type == TypeId(PrimitiveType::UnspecifiedFloat as usize)
            && type_system::is_float_type(context, right_type))
        || (*right_type == TypeId(PrimitiveType::UnspecifiedFloat as usize)
            && type_system::is_float_type(context, left_type))
    {
        Ok(TypeId(PrimitiveType::Bool as usize))
    } else {
        Err(SemanticAnalysisError::OperationTypeMismatch {
            operator: operator.to_string(),
            left_type: left_type.clone(),
            right_type: right_type.clone(),
            location: *location,
        })
    }
}
