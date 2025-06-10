
use super::super::error::SemanticAnalysisError;
use super::helpers;
use slang_ir::ast::{BinaryExpr, BinaryOperator};
use slang_ir::Location;
use slang_shared::CompilationContext;
use slang_types::TypeId;

/// Type alias for result of semantic analysis operations
pub type SemanticResult = Result<TypeId, SemanticAnalysisError>;

/// Checks if a type is compatible with an arithmetic operation when both operands have the same type.
/// Boolean types are not allowed for any arithmetic operation.
/// String types are only allowed for the Add operator (concatenation).
/// Unit types and function types are not allowed for any arithmetic operation.
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
    // Check for types that are never allowed in arithmetic operations
    if helpers::is_boolean_type(type_id) 
        || helpers::is_unit_type(type_id) 
        || context.is_function_type(type_id) 
    {
        return Err(helpers::operation_type_mismatch_error(
            &operator.to_string(),
            type_id,
            type_id,
            location,
        ));
    }

    // String types are only allowed for addition (concatenation)
    if helpers::is_string_type(type_id) && operator != &BinaryOperator::Add {
        return Err(helpers::operation_type_mismatch_error(
            &operator.to_string(),
            type_id,
            type_id,
            location,
        ));
    }

    // All other types (numeric types and string addition) are allowed
    Ok(type_id.clone())
}

/// Checks if mixed-type arithmetic operations are allowed, particularly handling
/// unspecified literals that can be coerced to match the other operand's type.
/// This function handles type coercion for arithmetic operations when operands have different types.
/// 
/// ### Supported Cases
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
    // Handle unspecified integer literal with specific integer type
    if helpers::is_unspecified_integer_type(left_type) && is_integer_type(context, right_type) {
        return check_unspecified_int_for_type(context, &bin_expr.left, right_type);
    }

    if helpers::is_unspecified_integer_type(right_type) && is_integer_type(context, left_type) {
        return check_unspecified_int_for_type(context, &bin_expr.right, left_type);
    }

    // Handle unspecified float literal with specific float type
    if helpers::is_unspecified_float_type(left_type) && is_float_type(context, right_type) {
        return check_unspecified_float_for_type(context, &bin_expr.left, right_type);
    }

    if helpers::is_unspecified_float_type(right_type) && is_float_type(context, left_type) {
        return check_unspecified_float_for_type(context, &bin_expr.right, left_type);
    }

    // Handle string concatenation
    if bin_expr.operator == BinaryOperator::Add
        && helpers::is_string_type(left_type)
        && helpers::is_string_type(right_type)
    {
        return Ok(left_type.clone());
    }

    // If none of the above cases apply, the operation is not allowed
    Err(helpers::operation_type_mismatch_error(
        &bin_expr.operator.to_string(),
        left_type,
        right_type,
        &bin_expr.location,
    ))
}

/// Helper function to check if a type is an integer type.
/// This includes all signed and unsigned integer types but not unspecified integers.
///
/// ### Arguments
/// * `context` - The compilation context
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is a specific integer type
/// * `false` otherwise
fn is_integer_type(context: &CompilationContext, type_id: &TypeId) -> bool {
    helpers::is_numeric_type(context, type_id) && !helpers::is_unspecified_integer_type(type_id) && !is_float_type(context, type_id)
}

/// Helper function to check if a type is a float type.
/// This includes all floating-point types but not unspecified floats.
///
/// ### Arguments
/// * `_context` - The compilation context (unused but kept for API consistency)
/// * `type_id` - The type to check
///
/// ### Returns
/// * `true` if the type is a specific float type
/// * `false` otherwise
fn is_float_type(_context: &CompilationContext, type_id: &TypeId) -> bool {
    use slang_types::PrimitiveType;
    
    if let Some(primitive) = PrimitiveType::from_int(type_id.0) {
        matches!(primitive, PrimitiveType::F32 | PrimitiveType::F64)
    } else {
        false
    }
}

/// Checks if an unspecified integer literal is in the valid range for a target type.
/// This is used when coercing an integer literal to a specific integer type.
///
/// ### Arguments
/// * `context` - The compilation context
/// * `expr` - The expression that might contain an unspecified integer literal
/// * `target_type` - The target integer type to coerce to
///
/// ### Returns
/// * `Ok(target_type)` if the coercion is valid
/// * `Err` with a range error if the literal is out of bounds
fn check_unspecified_int_for_type(
    context: &CompilationContext,
    expr: &slang_ir::ast::Expression,
    target_type: &TypeId,
) -> SemanticResult {
    // For now, we'll delegate to the type system module
    // In a future refactor, this logic could be moved here for better encapsulation
    super::super::type_system::check_unspecified_int_for_type(context, expr, target_type)
}

/// Checks if an unspecified float literal is in the valid range for a target type.
/// This is used when coercing a float literal to a specific float type.
///
/// ### Arguments
/// * `context` - The compilation context
/// * `expr` - The expression that might contain an unspecified float literal
/// * `target_type` - The target float type to coerce to
///
/// ### Returns
/// * `Ok(target_type)` if the coercion is valid
/// * `Err` with a range error if the literal is out of bounds
fn check_unspecified_float_for_type(
    context: &CompilationContext,
    expr: &slang_ir::ast::Expression,
    target_type: &TypeId,
) -> SemanticResult {
    // For now, we'll delegate to the type system module
    // In a future refactor, this logic could be moved here for better encapsulation
    super::super::type_system::check_unspecified_float_for_type(context, expr, target_type)
}
