use super::super::{error::SemanticAnalysisError, traits::SemanticResult};
use super::helpers;
use slang_ir::Location;
use slang_ir::ast::{Expression, LiteralValue, UnaryExpr, UnaryOperator};
use slang_shared::CompilationContext;
use slang_types::TypeId;

use super::super::type_system;

/// Checks if a unary operation is valid for the given operand type.
/// Handles both arithmetic negation (-) and logical negation (!).
///
/// ### Arguments
/// * `context` - The compilation context
/// * `unary_expr` - The unary expression to check
/// * `operand_type` - The type of the operand
///
/// ### Returns
/// * `Ok(type_id)` with the resulting type if the operation is valid
/// * `Err` with a descriptive error message if the operation is invalid
pub fn check_unary_operation(
    context: &CompilationContext,
    unary_expr: &UnaryExpr,
    operand_type: &TypeId,
) -> SemanticResult {
    match unary_expr.operator {
        UnaryOperator::Negate => check_negation_operation(context, unary_expr, operand_type),
        UnaryOperator::Not => check_logical_not_operation(operand_type, &unary_expr.location),
    }
}

/// Checks if arithmetic negation (-) is valid for the given operand type.
/// Supports signed integers, floats, and unspecified numeric literals.
/// Unsigned integers cannot be negated.
///
/// ### Arguments
/// * `context` - The compilation context
/// * `unary_expr` - The unary negation expression
/// * `operand_type` - The type of the operand
///
/// ### Returns
/// * `Ok(type_id)` with the operand type if negation is valid
/// * `Err` with a descriptive error message if negation is invalid
pub fn check_negation_operation(
    context: &CompilationContext,
    unary_expr: &UnaryExpr,
    operand_type: &TypeId,
) -> SemanticResult {
    // Handle unspecified integer literals
    if *operand_type == TypeId::unspecified_int() {
        if let Expression::Literal(lit) = &*unary_expr.right {
            if let LiteralValue::UnspecifiedInteger(_value) = &lit.value {
                return Ok(TypeId::unspecified_int());
            }
            return Ok(TypeId::unspecified_float());
        }
    }

    // Handle unspecified float literals
    if *operand_type == TypeId::unspecified_float() {
        if let Expression::Literal(_) = &*unary_expr.right {
            return Ok(TypeId::unspecified_float());
        }
    }

    // Check if the type is numeric
    let is_numeric = type_system::is_integer_type(context, operand_type)
        || type_system::is_float_type(context, operand_type);

    if is_numeric {
        // Signed types can be negated
        if is_signed_numeric_type(operand_type) {
            return Ok(operand_type.clone());
        }

        // Unsigned types cannot be negated
        if is_unsigned_integer_type(operand_type) {
            return Err(SemanticAnalysisError::InvalidUnaryOperation {
                operator: "-".to_string(),
                operand_type: operand_type.clone(),
                location: unary_expr.location,
            });
        }
    }

    Err(SemanticAnalysisError::InvalidUnaryOperation {
        operator: "-".to_string(),
        operand_type: operand_type.clone(),
        location: unary_expr.location,
    })
}

/// Checks if logical negation (!) is valid for the given operand type.
/// Only boolean types can be logically negated.
///
/// ### Arguments
/// * `operand_type` - The type of the operand
/// * `location` - The source location of the operation
///
/// ### Returns
/// * `Ok(bool_type)` if the operand is boolean
/// * `Err` with a descriptive error message if the operand is not boolean
pub fn check_logical_not_operation(operand_type: &TypeId, location: &Location) -> SemanticResult {
    if helpers::is_boolean_type(operand_type) {
        Ok(helpers::bool_type())
    } else {
        Err(SemanticAnalysisError::InvalidUnaryOperation {
            operator: "!".to_string(),
            operand_type: operand_type.clone(),
            location: *location,
        })
    }
}

/// Checks if a type is a signed numeric type that can be negated.
/// Includes signed integers (i32, i64) and floating point types (f32, f64).
///
/// ### Arguments
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is a signed numeric type
/// * `false` otherwise
pub fn is_signed_numeric_type(type_id: &TypeId) -> bool {
    *type_id == TypeId::i32()
        || *type_id == TypeId::i64()
        || *type_id == TypeId::f32()
        || *type_id == TypeId::f64()
}

/// Checks if a type is an unsigned integer type that cannot be negated.
/// Includes u32 and u64 types.
///
/// ### Arguments
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is an unsigned integer type
/// * `false` otherwise
pub fn is_unsigned_integer_type(type_id: &TypeId) -> bool {
    *type_id == TypeId::u32() || *type_id == TypeId::u64()
}
