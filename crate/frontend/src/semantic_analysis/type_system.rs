use super::error::SemanticAnalysisError;
use slang_ir::ast::{BinaryExpr, BinaryOperator, Expression, LetStatement, LiteralValue, UnaryOperator};
use slang_shared::CompilationContext;
use slang_types::{TYPE_NAME_U32, TYPE_NAME_U64, TypeId};

/// Type alias for result of type system operations
pub type SemanticResult = Result<TypeId, SemanticAnalysisError>;

/// Checks if a type is an integer type
///
/// ### Arguments
/// * `context` - The compilation context
/// * `type_id` - The type ID to check
///
/// ### Returns
/// True if the type is an integer type, false otherwise
pub fn is_integer_type(context: &CompilationContext, type_id: &TypeId) -> bool {
    context.is_integer_type(type_id)
}

/// Checks if a type is a float type
///
/// ### Arguments
/// * `context` - The compilation context
/// * `type_id` - The type ID to check
///
/// ### Returns
/// True if the type is a float type, false otherwise
pub fn is_float_type(context: &CompilationContext, type_id: &TypeId) -> bool {
    context.is_float_type(type_id)
}

/// Checks if a type is an unsigned integer type
///
/// ### Arguments
/// * `context` - The compilation context
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is u32 or u64, `false` otherwise
pub fn is_unsigned_type(context: &CompilationContext, type_id: &TypeId) -> bool {
    let type_name = context.get_type_name(type_id);
    type_name == TYPE_NAME_U64 || type_name == TYPE_NAME_U32
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
    if let Expression::Unary(unary_expr) = expr {
        if unary_expr.operator == UnaryOperator::Negate {
            if let Expression::Literal(lit) = &*unary_expr.right {
                if let LiteralValue::UnspecifiedInteger(n) = &lit.value {
                    if context.get_type_name(target_type) == "u32"
                        || context.get_type_name(target_type) == "u64"
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

/// Converts unspecified literal types to concrete types.
/// This is used to assign a default concrete type when an unspecified literal
/// is used in a context where the type wasn't explicitly given.
///
/// ### Arguments
/// * `type_id` - The type to finalize
///
/// ### Returns
/// * The concrete type (i64 for unspecified integers, f64 for unspecified floats)
/// * The original type if it wasn't an unspecified literal type
pub fn finalize_inferred_type(type_id: TypeId) -> TypeId {
    if type_id == TypeId::unspecified_int() {
        TypeId::i64()
    } else if type_id == TypeId::unspecified_float() {
        TypeId::f64()
    } else {
        type_id
    }
}

/// Determines the final type of a variable in a let statement based on both the
/// declared type (if any) and the initialization expression's type.
/// Handles type inference and coercion of unspecified literals.
///
/// ### Arguments
/// * `context` - The compilation context
/// * `let_stmt` - The let statement being analyzed
/// * `expr_type` - The type of the initialization expression
///
/// ### Returns
/// * `Ok(type_id)` with the final determined type if valid
/// * `Err` with a SemanticAnalysisError if there's a type mismatch
pub fn determine_let_statement_type(
    context: &CompilationContext,
    let_stmt: &LetStatement,
    expr_type: TypeId,
) -> SemanticResult {
    if let_stmt.expr_type == TypeId::unknown() {
        return Ok(expr_type);
    }

    if let_stmt.expr_type == expr_type {
        if is_unsigned_type(context, &let_stmt.expr_type) {
            check_unspecified_int_for_type(context, &let_stmt.value, &let_stmt.expr_type)?;
        }
        return Ok(let_stmt.expr_type.clone());
    }

    if context.get_function_type(&let_stmt.expr_type).is_some() && 
       context.get_function_type(&expr_type).is_some() {
        if let_stmt.expr_type == expr_type {
            return Ok(let_stmt.expr_type.clone());
        } else {
            return Err(SemanticAnalysisError::TypeMismatch {
                expected: let_stmt.expr_type.clone(),
                actual: expr_type,
                context: Some(let_stmt.name.clone()),
                location: let_stmt.location,
            });
        }
    }

    if expr_type == TypeId::unspecified_int() {
        return handle_unspecified_int_assignment(context, let_stmt, &expr_type);
    }

    if expr_type == TypeId::unspecified_float() {
        return handle_unspecified_float_assignment(context, let_stmt, &expr_type);
    }

    Err(SemanticAnalysisError::TypeMismatch {
        expected: let_stmt.expr_type.clone(),
        actual: expr_type,
        context: Some(let_stmt.name.clone()),
        location: let_stmt.location,
    })
}

/// Handles assignment of an unspecified integer literal to a variable with a declared type.
///
/// ### Arguments
/// * `context` - The compilation context
/// * `let_stmt` - The let statement being analyzed.
/// * `expr_type` - The type of the initialization expression (should be unspecified_int_type).
///
/// ### Returns
/// * `Ok(type_id)` with the declared type if the literal is valid for that type.
/// * `Err` with a SemanticAnalysisError if there's a type mismatch or value out of range.
pub fn handle_unspecified_int_assignment(
    context: &CompilationContext,
    let_stmt: &LetStatement,
    _expr_type: &TypeId,
) -> SemanticResult {
    if is_integer_type(context, &let_stmt.expr_type) {
        check_unspecified_int_for_type(context, &let_stmt.value, &let_stmt.expr_type)
    } else {
        Err(SemanticAnalysisError::TypeMismatch {
            expected: let_stmt.expr_type.clone(),
            actual: TypeId::unspecified_int(),
            context: Some(let_stmt.name.clone()),
            location: let_stmt.location,
        })
    }
}

/// Handles assignment of an unspecified float literal to a variable with a declared type.
///
/// ### Arguments
/// * `context` - The compilation context
/// * `let_stmt` - The let statement being analyzed.
/// * `expr_type` - The type of the initialization expression (should be unspecified_float_type).
///
/// ### Returns
/// * `Ok(type_id)` with the declared type if the literal is valid for that type.
/// * `Err` with a SemanticAnalysisError if there's a type mismatch or value out of range.
pub fn handle_unspecified_float_assignment(
    context: &CompilationContext,
    let_stmt: &LetStatement,
    _expr_type: &TypeId,
) -> SemanticResult {
    if is_float_type(context, &let_stmt.expr_type) {
        check_unspecified_float_for_type(context, &let_stmt.value, &let_stmt.expr_type)
    } else {
        Err(SemanticAnalysisError::TypeMismatch {
            expected: let_stmt.expr_type.clone(),
            actual: TypeId::unspecified_float(),
            context: Some(let_stmt.name.clone()),
            location: let_stmt.location,
        })
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
    if *left_type == TypeId::unspecified_int()
        && is_integer_type(context, right_type)
    {
        return check_unspecified_int_for_type(context, &bin_expr.left, right_type);
    }

    if *right_type == TypeId::unspecified_int()
        && is_integer_type(context, left_type)
    {
        return check_unspecified_int_for_type(context, &bin_expr.right, left_type);
    }

    if *left_type == TypeId::unspecified_float()
        && is_float_type(context, right_type)
    {
        return check_unspecified_float_for_type(context, &bin_expr.left, right_type);
    }

    if *right_type == TypeId::unspecified_float()
        && is_float_type(context, left_type)
    {
        return check_unspecified_float_for_type(context, &bin_expr.right, left_type);
    }

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
