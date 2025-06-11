use slang_shared::CompilationContext;
use slang_types::TypeId;
use slang_ir::ast::{BinaryExpr, Expression, LetStatement};

use super::{
    TypeChecker,
    TypeCoercion,
    TypeInference,
    TypeValidation,
    inference::{finalize_inferred_type, determine_let_statement_type},
    coercion::{check_unspecified_int_for_type, check_unspecified_float_for_type},
};
use super::super::traits::SemanticResult;

/// Coordinates between specialized type checking modules
/// 
/// This struct provides a unified interface for type checking operations
/// while delegating to specialized modules for specific concerns like
/// inference, coercion, and validation.
pub struct TypeCheckingCoordinator<'a> {
    checker: TypeChecker<'a>,
    coercion: TypeCoercion<'a>,
    _inference: TypeInference, // Unused for now but kept for future extensions
    validation: TypeValidation<'a>,
}

impl<'a> TypeCheckingCoordinator<'a> {
    /// Create a new type checking coordinator
    /// 
    /// # Arguments
    /// * `context` - The compilation context for type information
    pub fn new(context: &'a CompilationContext) -> Self {
        Self {
            checker: TypeChecker::new(context),
            coercion: TypeCoercion::new(context),
            _inference: TypeInference{},
            validation: TypeValidation::new(context),
        }
    }

    /// Check if two types are compatible for assignment with coercion
    /// 
    /// # Arguments
    /// * `target` - The target type (left side of assignment)
    /// * `source` - The source type (right side of assignment)
    /// 
    /// # Returns
    /// `true` if assignment is allowed, `false` otherwise
    pub fn check_assignment_compatibility(&self, target: &TypeId, source: &TypeId) -> bool {
        self.checker.check_assignment_compatibility(target, source)
    }

    /// Check if a function call is valid and return the result type
    /// 
    /// # Arguments
    /// * `function_type` - The function's type signature
    /// * `argument_types` - The types of the provided arguments
    /// 
    /// # Returns
    /// Result containing the return type or an error
    pub fn check_function_call(
        &self,
        function_type: &TypeId,
        argument_types: &[TypeId],
    ) -> SemanticResult {
        self.checker.check_function_call(function_type, argument_types)
    }

    /// Check if mixed-type arithmetic operations are allowed with coercion
    /// 
    /// # Arguments
    /// * `left_type` - The type of the left operand
    /// * `right_type` - The type of the right operand
    /// * `bin_expr` - The binary expression containing both operands and the operator
    /// 
    /// # Returns
    /// Result containing the operation result type or an error
    pub fn check_mixed_arithmetic_with_coercion(
        &self,
        left_type: &TypeId,
        right_type: &TypeId,
        bin_expr: &BinaryExpr,
    ) -> SemanticResult {
        self.coercion.check_mixed_arithmetic_operation(left_type, right_type, bin_expr)
    }

    /// Determine the final type for a let statement with potential inference
    /// 
    /// # Arguments
    /// * `let_stmt` - The let statement being analyzed
    /// * `expr_type` - The type of the initialization expression
    /// 
    /// # Returns
    /// Result containing the final determined type
    pub fn determine_let_statement_type(
        &self,
        let_stmt: &LetStatement,
        expr_type: TypeId,
    ) -> SemanticResult {
        determine_let_statement_type(self.checker.context(), let_stmt, expr_type)
    }

    /// Finalize an inferred type (convert unspecified literals to concrete types)
    /// 
    /// # Arguments
    /// * `type_id` - The type to finalize
    /// 
    /// # Returns
    /// The concrete type (i64 for unspecified integers, f64 for unspecified floats)
    pub fn finalize_inferred_type(&self, type_id: TypeId) -> TypeId {
        finalize_inferred_type(type_id)
    }

    /// Validate that a literal value is within range for its target type
    /// 
    /// # Arguments
    /// * `expr` - The expression containing the literal
    /// * `target_type` - The target type to validate against
    /// 
    /// # Returns
    /// Result indicating if validation passed
    pub fn validate_literal_range(
        &self,
        expr: &Expression,
        target_type: &TypeId,
    ) -> SemanticResult {
        // Use coercion module's range checking capabilities
        if self.is_integer_type(target_type) {
            check_unspecified_int_for_type(self.checker.context(), expr, target_type)
        } else if self.is_float_type(target_type) {
            check_unspecified_float_for_type(self.checker.context(), expr, target_type)
        } else {
            Ok(target_type.clone())
        }
    }

    /// Validate function declaration constraints
    /// 
    /// # Arguments
    /// * `func_decl` - The function declaration to validate
    /// 
    /// # Returns
    /// Result indicating if validation passed
    pub fn validate_function_declaration(
        &self,
        func_decl: &slang_ir::ast::FunctionDeclarationStmt,
    ) -> SemanticResult {
        self.validation.validate_function_declaration(func_decl)
    }

    /// Check if a type is numeric
    pub fn is_numeric_type(&self, type_id: &TypeId) -> bool {
        self.checker.is_numeric_type(type_id)
    }

    /// Check if a type is an integer type
    pub fn is_integer_type(&self, type_id: &TypeId) -> bool {
        self.checker.is_integer_type(type_id)
    }

    /// Check if a type is a float type
    pub fn is_float_type(&self, type_id: &TypeId) -> bool {
        self.checker.is_float_type(type_id)
    }

    /// Check if a type is an unsigned integer type
    pub fn is_unsigned_integer_type(&self, type_id: &TypeId) -> bool {
        self.checker.is_unsigned_integer_type(type_id)
    }

    /// Check if an unspecified literal can be coerced to a target type
    pub fn can_coerce_unspecified_literal(&self, source: &TypeId, target: &TypeId) -> bool {
        self.coercion.can_coerce(source, target)
    }
}
