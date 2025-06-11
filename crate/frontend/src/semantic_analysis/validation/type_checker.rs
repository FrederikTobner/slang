use slang_shared::CompilationContext;
use slang_types::TypeId;

use super::super::{
    traits::SemanticResult,
    error::SemanticAnalysisError,
};

/// Central type checking coordination
/// 
/// Provides high-level type checking operations that coordinate between
/// different specialized checking strategies like coercion, inference,
/// and validation.
pub struct TypeChecker<'a> {
    context: &'a CompilationContext,
}

impl<'a> TypeChecker<'a> {
    /// Creates a new type checker
    /// 
    /// # Arguments
    /// * `context` - The compilation context for type information
    pub fn new(context: &'a CompilationContext) -> Self {
        Self { context }
    }

    /// Get access to the compilation context
    /// 
    /// # Returns
    /// Reference to the compilation context
    pub fn context(&self) -> &'a CompilationContext {
        self.context
    }

    /// Checks if two types are compatible for assignment
    /// 
    /// # Arguments
    /// * `target` - The target type (left side of assignment)
    /// * `source` - The source type (right side of assignment)
    /// 
    /// # Returns
    /// `true` if assignment is allowed, `false` otherwise
    pub fn check_assignment_compatibility(&self, target: &TypeId, source: &TypeId) -> bool {
        // Exact type match
        if target == source {
            return true;
        }

        // Check for unspecified literal coercion
        if self.can_coerce_unspecified_literal(source, target) {
            return true;
        }

        false
    }

    /// Checks if a function call is valid
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
        if let Some(func_type) = self.context.get_function_type(function_type) {
            // Check parameter count
            if func_type.param_types.len() != argument_types.len() {
                return Err(SemanticAnalysisError::ArgumentCountMismatch {
                    function_name: "unknown".to_string(), // TODO: Pass function name from caller
                    expected: func_type.param_types.len(),
                    actual: argument_types.len(),
                    location: slang_ir::location::Location::default(),
                });
            }

            // Check parameter types
            for (i, (expected, actual)) in func_type.param_types.iter().zip(argument_types).enumerate() {
                if !self.check_assignment_compatibility(expected, actual) {
                    return Err(SemanticAnalysisError::ArgumentTypeMismatch {
                        function_name: "unknown".to_string(), // TODO: Pass function name from caller
                        argument_position: i + 1,
                        expected: expected.clone(),
                        actual: actual.clone(),
                        location: slang_ir::location::Location::default(),
                    });
                }
            }

            Ok(func_type.return_type.clone())
        } else {
            Err(SemanticAnalysisError::UndefinedFunction {
                name: "unknown".to_string(), // TODO: Pass function name from caller
                location: slang_ir::location::Location::default(),
            })
        }
    }

    /// Checks if an unspecified literal can be coerced to a target type
    /// 
    /// # Arguments
    /// * `source` - The source type (unspecified literal)
    /// * `target` - The target type
    /// 
    /// # Returns
    /// `true` if coercion is possible, `false` otherwise
    pub fn can_coerce_unspecified_literal(&self, source: &TypeId, target: &TypeId) -> bool {
        // Unspecified int to specific integer types
        if *source == TypeId::unspecified_int() {
            return self.context.is_integer_type(target);
        }

        // Unspecified float to specific float types
        if *source == TypeId::unspecified_float() {
            return self.context.is_float_type(target);
        }

        false
    }

    /// Checks if a type is numeric
    /// 
    /// # Arguments
    /// * `type_id` - The type to check
    /// 
    /// # Returns
    /// `true` if the type is numeric, `false` otherwise
    pub fn is_numeric_type(&self, type_id: &TypeId) -> bool {
        self.context.is_numeric_type(type_id)
    }

    /// Checks if a type is an integer type
    /// 
    /// # Arguments
    /// * `type_id` - The type to check
    /// 
    /// # Returns
    /// `true` if the type is an integer, `false` otherwise
    pub fn is_integer_type(&self, type_id: &TypeId) -> bool {
        self.context.is_integer_type(type_id)
    }

    /// Checks if a type is a float type
    /// 
    /// # Arguments
    /// * `type_id` - The type to check
    /// 
    /// # Returns
    /// `true` if the type is a float, `false` otherwise
    pub fn is_float_type(&self, type_id: &TypeId) -> bool {
        self.context.is_float_type(type_id)
    }

    /// Checks if a type is an unsigned integer type
    /// 
    /// # Arguments
    /// * `type_id` - The type to check
    /// 
    /// # Returns
    /// `true` if the type is unsigned integer, `false` otherwise
    pub fn is_unsigned_integer_type(&self, type_id: &TypeId) -> bool {
        self.context.is_unsigned_integer_type(type_id)
    }
}
