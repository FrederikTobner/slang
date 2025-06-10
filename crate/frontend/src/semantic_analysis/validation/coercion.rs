use slang_ir::ast::{BinaryExpr, BinaryOperator, Expression, LiteralValue, UnaryOperator};
use slang_shared::CompilationContext;
use slang_types::{TypeId, TYPE_NAME_U32, TYPE_NAME_U64};

use super::super::{
    traits::SemanticResult,
    error::SemanticAnalysisError,
};

/// Handles all type coercion rules and operations
/// 
/// This module is responsible for determining when and how types can be
/// automatically converted or coerced, particularly for unspecified literal types.
pub struct TypeCoercion<'a> {
    context: &'a CompilationContext,
}

impl<'a> TypeCoercion<'a> {
    /// Creates a new type coercion handler
    /// 
    /// # Arguments
    /// * `context` - The compilation context for type information
    pub fn new(context: &'a CompilationContext) -> Self {
        Self { context }
    }

    /// Checks if an unspecified literal can be coerced to a target type
    /// 
    /// # Arguments
    /// * `source_type` - The unspecified literal type
    /// * `target_type` - The target type to coerce to
    /// 
    /// # Returns
    /// `true` if coercion is possible, `false` otherwise
    pub fn can_coerce(&self, source_type: &TypeId, target_type: &TypeId) -> bool {
        // Unspecified int to specific integer types
        if *source_type == TypeId::unspecified_int() {
            return self.context.is_integer_type(target_type);
        }

        // Unspecified float to specific float types
        if *source_type == TypeId::unspecified_float() {
            return self.context.is_float_type(target_type);
        }

        false
    }

    /// Checks for mixed-type arithmetic operations with coercion
    /// 
    /// Handles cases where unspecified literals can be coerced to match
    /// the other operand's type in arithmetic operations.
    /// 
    /// # Arguments
    /// * `left_type` - Type of the left operand
    /// * `right_type` - Type of the right operand
    /// * `bin_expr` - The binary expression for context
    /// 
    /// # Returns
    /// Result with the resulting type or an error
    pub fn check_mixed_arithmetic_operation(
        &self,
        left_type: &TypeId,
        right_type: &TypeId,
        bin_expr: &BinaryExpr,
    ) -> SemanticResult {
        // Left operand is unspecified int, right is specific integer
        if *left_type == TypeId::unspecified_int()
            && self.context.is_integer_type(right_type)
        {
            return check_unspecified_int_for_type(self.context, &bin_expr.left, right_type);
        }

        // Right operand is unspecified int, left is specific integer
        if *right_type == TypeId::unspecified_int()
            && self.context.is_integer_type(left_type)
        {
            return check_unspecified_int_for_type(self.context, &bin_expr.right, left_type);
        }

        // Left operand is unspecified float, right is specific float
        if *left_type == TypeId::unspecified_float()
            && self.context.is_float_type(right_type)
        {
            return check_unspecified_float_for_type(self.context, &bin_expr.left, right_type);
        }

        // Right operand is unspecified float, left is specific float
        if *right_type == TypeId::unspecified_float()
            && self.context.is_float_type(left_type)
        {
            return check_unspecified_float_for_type(self.context, &bin_expr.right, left_type);
        }

        // String concatenation
        if bin_expr.operator == BinaryOperator::Add
            && *left_type == TypeId::string()
            && *right_type == TypeId::string()
        {
            return Ok(TypeId::string());
        }

        Err(SemanticAnalysisError::OperationTypeMismatch {
            operator: bin_expr.operator.to_string(),
            left_type: left_type.clone(),
            right_type: right_type.clone(),
            location: bin_expr.location,
        })
    }
}

/// Checks if an unspecified integer literal is in the valid range for a target type.
/// This is used when coercing an integer literal to a specific integer type.
///
/// ### Arguments
/// * `context` - The compilation context
/// * `expr` - The expression that might contain an unspecified integer literal
/// * `target_type` - The specific integer type to check against
///
/// ### Returns
/// * `Ok(target_type)` if the literal is in range for the target type
/// * `Err` with a descriptive error message if the literal is out of range
/// * `Ok(target_type)` if the expression isn't an unspecified integer literal
pub fn check_unspecified_int_for_type(
    context: &CompilationContext,
    expr: &Expression,
    target_type: &TypeId,
) -> SemanticResult {
    // Handle negative unspecified integers (unary negation)
    if let Expression::Unary(unary_expr) = expr {
        if unary_expr.operator == UnaryOperator::Negate {
            if let Expression::Literal(lit) = &*unary_expr.right {
                if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                    // Unsigned types cannot hold negative values
                    if context.get_type_name(target_type) == TYPE_NAME_U32
                        || context.get_type_name(target_type) == TYPE_NAME_U64
                    {
                        return Err(SemanticAnalysisError::ValueOutOfRange {
                            value: format!("-{}", n),
                            target_type: target_type.clone(),
                            is_float: false,
                            location: expr.location(),
                        });
                    }
                }
            }
        }
    }

    // Handle positive unspecified integers
    if let Expression::Literal(lit) = expr {
        if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
            let value_in_range = context.check_value_in_range(n, target_type);

            if value_in_range {
                return Ok(target_type.clone());
            } else {
                return Err(SemanticAnalysisError::ValueOutOfRange {
                    value: n.to_string(),
                    target_type: target_type.clone(),
                    is_float: false,
                    location: expr.location(),
                });
            }
        }
    }
    Ok(target_type.clone())
}

/// Checks if an unspecified float literal is in the valid range for a target type.
/// This is used when coercing a float literal to a specific floating-point type.
///
/// ### Arguments
/// * `context` - The compilation context
/// * `expr` - The expression that might contain an unspecified float literal
/// * `target_type` - The specific float type to check against (e.g., f32, f64)
///
/// ### Returns
/// * `Ok(target_type)` if the literal is in range for the target type
/// * `Err` with a descriptive error message if the literal is out of range
/// * `Ok(target_type)` if the expression isn't an unspecified float literal
pub fn check_unspecified_float_for_type(
    context: &CompilationContext,
    expr: &Expression,
    target_type: &TypeId,
) -> SemanticResult {
    if let Expression::Literal(lit) = expr {
        if let LiteralValue::UnspecifiedFloat(f) = &lit.value {
            let value_in_range = context.check_float_value_in_range(f, target_type);

            if value_in_range {
                return Ok(target_type.clone());
            } else {
                return Err(SemanticAnalysisError::ValueOutOfRange {
                    value: f.to_string(),
                    target_type: target_type.clone(),
                    is_float: true,
                    location: expr.location(),
                });
            }
        }
    }
    Ok(target_type.clone())
}
