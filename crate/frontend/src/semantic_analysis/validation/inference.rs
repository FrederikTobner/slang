use slang_ir::ast::{LetStatement, LiteralValue};
use slang_shared::CompilationContext;
use slang_types::{TypeId, TYPE_NAME_U32, TYPE_NAME_U64};

use super::super::{
    traits::SemanticResult,
    error::SemanticAnalysisError,
};
use super::coercion::{check_unspecified_int_for_type, check_unspecified_float_for_type};

/// Handles type inference and finalization
/// 
/// This module is responsible for inferring types when they are not explicitly
/// specified and finalizing unspecified literal types to concrete types.
pub struct TypeInference{
}

impl TypeInference{


    /// Infers the type of a literal expression
    /// 
    /// # Arguments
    /// * `literal_value` - The literal value to infer type for
    /// 
    /// # Returns
    /// The inferred TypeId for the literal
    pub fn infer_literal_type(&self, literal_value: &LiteralValue) -> TypeId {
        match literal_value {
            LiteralValue::Boolean(_) => TypeId::bool(),
            LiteralValue::UnspecifiedInteger(_) => TypeId::unspecified_int(),
            LiteralValue::UnspecifiedFloat(_) => TypeId::unspecified_float(),
            LiteralValue::I32(_) => TypeId::i32(),
            LiteralValue::I64(_) => TypeId::i64(),
            LiteralValue::U32(_) => TypeId::u32(),
            LiteralValue::U64(_) => TypeId::u64(),
            LiteralValue::F32(_) => TypeId::f32(),
            LiteralValue::F64(_) => TypeId::f64(),
            LiteralValue::String(_) => TypeId::string(),
            LiteralValue::Unit => TypeId::unit(),
        }
    }

    /// Checks if a type requires special handling for unsigned integer assignment
    /// 
    /// # Arguments
    /// * `type_id` - The type to check
    /// 
    /// # Returns
    /// `true` if the type is unsigned integer, `false` otherwise
    pub fn is_unsigned_type(&self, type_id: &TypeId) -> bool {
        type_id == &TypeId::u32() ||
        type_id == &TypeId::u64()
    }
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
    // Type inference case - no explicit type annotation
    if let_stmt.expr_type == TypeId::unknown() {
        return Ok(expr_type);
    }

    // Exact type match
    if let_stmt.expr_type == expr_type {
        // Special validation for unsigned types to catch negative values early
        if is_unsigned_type(context, &let_stmt.expr_type) {
            check_unspecified_int_for_type(context, &let_stmt.value, &let_stmt.expr_type)?;
        }
        return Ok(let_stmt.expr_type.clone());
    }

    // Function type compatibility check
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

    // Handle unspecified integer assignment
    if expr_type == TypeId::unspecified_int() {
        return handle_unspecified_int_assignment(context, let_stmt, &expr_type);
    }

    // Handle unspecified float assignment
    if expr_type == TypeId::unspecified_float() {
        return handle_unspecified_float_assignment(context, let_stmt, &expr_type);
    }

    // No valid coercion possible
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
/// * `_expr_type` - The type of the initialization expression (should be unspecified_int_type).
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
/// * `_expr_type` - The type of the initialization expression (should be unspecified_float_type).
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
