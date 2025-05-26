use crate::error::CompilerError;
use slang_ir::SourceLocation;
use slang_types::TypeId;
use slang_compilation_context::CompilationContext;

/// Represents different categories of semantic analysis errors
/// that occur during static analysis of the program.
///
/// Each variant contains the necessary context for generating
/// appropriate error messages that maintain the existing format.
#[derive(Debug, Clone)]
pub enum SemanticAnalysisError {
    /// An attempt to use a variable that has not been defined in scope
    UndefinedVariable {
        /// The name of the undefined variable
        name: String,
        /// The source location where the error occurred
        location: SourceLocation,
    },

    /// A variable with the same name is already defined in the current scope
    VariableRedefinition {
        /// The name of the variable being redefined
        name: String,
        /// The source location where the redefinition occurred
        location: SourceLocation,
    },

    /// A symbol (type, variable, function) is being redefined.
    SymbolRedefinition {
        /// The name of the symbol being redefined
        name: String,
        /// The kind of the symbol (e.g., type, variable, function)
        kind: String,
        /// The source location where the redefinition occurred
        location: SourceLocation,
    },

    /// A struct field is defined with an invalid type (e.g. unknown, unspecified)
    InvalidFieldType {
        /// The name of the struct containing the field
        struct_name: String,
        /// The name of the field with the invalid type
        field_name: String,
        /// The type ID of the invalid field type
        type_id: TypeId,
        /// The source location where the invalid field type was defined
        location: SourceLocation,
    },

    /// The type of an expression does not match the expected type
    TypeMismatch {
        /// The expected type
        expected: TypeId,
        /// The actual type found
        actual: TypeId,
        /// Optional context for the mismatch (like variable or function name)
        context: Option<String>,
        /// The source location where the type mismatch occurred
        location: SourceLocation,
    },

    /// Incompatible types for an operation like arithmetic or comparison
    OperationTypeMismatch {
        /// The operation being performed (e.g., +, -, *, /)
        operator: String,
        /// Left operand type
        left_type: TypeId,
        /// Right operand type
        right_type: TypeId,
        /// The source location where the operation type mismatch occurred
        location: SourceLocation,
    },

    /// Logical operators (AND, OR) used with non-boolean operands
    LogicalOperatorTypeMismatch {
        /// The logical operator being used (AND, OR)
        operator: String,
        /// Left operand type
        left_type: TypeId,
        /// Right operand type
        right_type: TypeId,
        /// The source location where the logical operator type mismatch occurred
        location: SourceLocation,
    },

    /// Value is out of range for the target type (e.g., integer overflow)
    ValueOutOfRange {
        /// The value that can't fit in the type
        value: String,
        /// The target type
        target_type: TypeId,
        /// Whether the value is an integer or float
        is_float: bool,
        /// The source location where the value out of range occurred
        location: SourceLocation,
    },

    /// Function call with wrong number of arguments
    ArgumentCountMismatch {
        /// Function name
        function_name: String,
        /// Expected number of arguments
        expected: usize,
        /// Actual number of arguments provided
        actual: usize,
        /// The source location where the argument count mismatch occurred
        location: SourceLocation,
    },

    /// Function call with wrong argument types
    ArgumentTypeMismatch {
        /// Function name
        function_name: String,
        /// Argument position (1-based)
        argument_position: usize,
        /// Expected type
        expected: TypeId,
        /// Actual type
        actual: TypeId,
        /// The source location where the argument type mismatch occurred
        location: SourceLocation,
    },

    /// Return statement outside of a function
    ReturnOutsideFunction {
        /// The source location where the return statement was found
        location: SourceLocation,
    },

    /// Return type does not match function declaration
    ReturnTypeMismatch {
        /// Expected return type
        expected: TypeId,
        /// Actual returned type
        actual: TypeId,
        /// The source location where the return type mismatch occurred
        location: SourceLocation,
    },

    /// Missing return value for a function that requires one
    MissingReturnValue {
        /// Expected return type
        expected: TypeId,
        /// The source location where the missing return value was found
        location: SourceLocation,
    },

    /// Undefined function in a function call
    UndefinedFunction {
        /// The name of the undefined function
        name: String,
        /// The source location where the undefined function was called
        location: SourceLocation,
    },

    /// Unary operation applied to incompatible type
    InvalidUnaryOperation {
        /// The unary operator (e.g., -, !)
        operator: String,
        /// The operand type
        operand_type: TypeId,
        /// The source location where the invalid unary operation occurred
        location: SourceLocation,
    },

    /// Assignment to an immutable variable
    AssignmentToImmutableVariable {
        /// The name of the immutable variable
        name: String,
        /// The source location where the assignment attempt occurred
        location: SourceLocation,
    },

    /// An expression has an unexpected form or context
    InvalidExpression {
        /// A description of what was expected vs what was found
        message: String,
        /// The source location where the invalid expression was found
        location: SourceLocation,
    },
}

impl SemanticAnalysisError {
    /// Convert the SemanticAnalysisError to a String representation
    /// that matches the existing error message formats.
    pub fn format_message(&self, context: &CompilationContext) -> String {
        match self {
            SemanticAnalysisError::UndefinedVariable { name, .. } => {
                format!("Undefined variable: {}", name)
            }

            SemanticAnalysisError::VariableRedefinition { name, .. } => {
                format!("Variable '{}' already defined", name)
            }

            SemanticAnalysisError::SymbolRedefinition { name, kind, .. } => {
                format!(
                    "Symbol '{}' of kind '{}' is already defined or conflicts with an existing symbol.",
                    name, kind
                )
            }

            SemanticAnalysisError::InvalidFieldType {
                struct_name,
                field_name,
                type_id,
                ..
            } => {
                format!(
                    "Invalid type '{}' for field '{}' in struct '{}'. Fields cannot be of unknown or unspecified type.",
                    context.get_type_name(type_id),
                    field_name,
                    struct_name
                )
            }

            SemanticAnalysisError::TypeMismatch {
                expected,
                actual,
                context: error_context,
                ..
            } => {
                if let Some(ctx) = error_context {
                    format!(
                        "Type mismatch: variable {} is {} but expression is {}",
                        ctx,
                        context.get_type_name(expected),
                        context.get_type_name(actual)
                    )
                } else {
                    format!(
                        "Type mismatch: expected {}, got {}",
                        context.get_type_name(expected),
                        context.get_type_name(actual)
                    )
                }
            }

            SemanticAnalysisError::OperationTypeMismatch {
                operator,
                left_type,
                right_type,
                ..
            } => {
                format!(
                    "Type mismatch: cannot apply '{}' operator on {} and {}",
                    operator,
                    context.get_type_name(left_type),
                    context.get_type_name(right_type)
                )
            }

            SemanticAnalysisError::LogicalOperatorTypeMismatch {
                operator,
                left_type,
                right_type,
                ..
            } => {
                format!(
                    "Logical operator '{}' requires boolean operands, got {} and {}",
                    operator,
                    context.get_type_name(left_type),
                    context.get_type_name(right_type)
                )
            }

            SemanticAnalysisError::ValueOutOfRange {
                value,
                target_type,
                is_float,
                ..
            } => {
                if *is_float {
                    format!(
                        "Float literal {} is out of range for type {}",
                        value,
                        context.get_type_name(target_type)
                    )
                } else {
                    format!(
                        "Integer literal {} is out of range for type {}",
                        value,
                        context.get_type_name(target_type)
                    )
                }
            }

            SemanticAnalysisError::ArgumentCountMismatch {
                function_name,
                expected,
                actual,
                ..
            } => {
                format!(
                    "Function '{}' expects {} arguments, but got {}",
                    function_name, expected, actual
                )
            }

            SemanticAnalysisError::ArgumentTypeMismatch {
                function_name,
                argument_position,
                expected,
                actual,
                ..
            } => {
                format!(
                    "Type mismatch: function '{}' expects argument {} to be {}, but got {}",
                    function_name,
                    argument_position,
                    context.get_type_name(expected),
                    context.get_type_name(actual)
                )
            }

            SemanticAnalysisError::ReturnOutsideFunction { .. } => {
                "Return statement outside of function".to_string()
            }

            SemanticAnalysisError::ReturnTypeMismatch {
                expected, actual, ..
            } => {
                format!(
                    "Type mismatch: function returns {} but got {}",
                    context.get_type_name(expected),
                    context.get_type_name(actual)
                )
            }

            SemanticAnalysisError::MissingReturnValue { expected, .. } => {
                format!(
                    "Type mismatch: function returns {} but no return value provided",
                    context.get_type_name(expected)
                )
            }

            SemanticAnalysisError::UndefinedFunction { name, .. } => {
                format!("Undefined function: {}", name)
            }

            SemanticAnalysisError::InvalidUnaryOperation {
                operator,
                operand_type,
                ..
            } => {
                if operator == "!" {
                    format!(
                        "Boolean not operator '!' can only be applied to boolean types, but got {}",
                        context.get_type_name(operand_type)
                    )
                } else if operator == "-" {
                    // Special handling for unsigned types to match existing error messages
                    if context.get_type_name(operand_type) == "u32"
                        || context.get_type_name(operand_type) == "u64"
                    {
                        "Cannot negate unsigned type".to_string()
                    } else {
                        format!(
                            "Cannot negate non-numeric type '{}'",
                            context.get_type_name(operand_type)
                        )
                    }
                } else {
                    format!(
                        "Cannot apply operator '{}' to type {}",
                        operator,
                        context.get_type_name(operand_type)
                    )
                }
            }

            SemanticAnalysisError::AssignmentToImmutableVariable { name, .. } => {
                format!("Cannot assign to immutable variable '{}'", name)
            }

            SemanticAnalysisError::InvalidExpression { message, .. } => message.clone(),
        }
    }

    /// Extracts the SourceLocation from any SemanticAnalysisError variant.
    fn get_location(&self) -> &SourceLocation {
        match self {
            SemanticAnalysisError::UndefinedVariable { location, .. } => location,
            SemanticAnalysisError::VariableRedefinition { location, .. } => location,
            SemanticAnalysisError::SymbolRedefinition { location, .. } => location,
            SemanticAnalysisError::InvalidFieldType { location, .. } => location,
            SemanticAnalysisError::TypeMismatch { location, .. } => location,
            SemanticAnalysisError::OperationTypeMismatch { location, .. } => location,
            SemanticAnalysisError::LogicalOperatorTypeMismatch { location, .. } => location,
            SemanticAnalysisError::ValueOutOfRange { location, .. } => location,
            SemanticAnalysisError::ArgumentCountMismatch { location, .. } => location,
            SemanticAnalysisError::ArgumentTypeMismatch { location, .. } => location,
            SemanticAnalysisError::ReturnOutsideFunction { location, .. } => location,
            SemanticAnalysisError::ReturnTypeMismatch { location, .. } => location,
            SemanticAnalysisError::MissingReturnValue { location, .. } => location,
            SemanticAnalysisError::UndefinedFunction { location, .. } => location,
            SemanticAnalysisError::InvalidUnaryOperation { location, .. } => location,
            SemanticAnalysisError::AssignmentToImmutableVariable { location, .. } => location,
            SemanticAnalysisError::InvalidExpression { location, .. } => location,
        }
    }

    /// Determines the token length for the error, preferring the length from SourceLocation.
    /// Falls back to heuristics if the SourceLocation length is not available or is 0.
    fn get_token_length(&self) -> Option<usize> {
        let location = self.get_location();

        // Prefer the length from SourceLocation if it's meaningful
        if location.length > 0 {
            return Some(location.length);
        }

        // Fall back to heuristics for backward compatibility
        match self {
            SemanticAnalysisError::UndefinedVariable { name, .. } => Some(name.len()),
            SemanticAnalysisError::VariableRedefinition { name, .. } => Some(name.len()),
            SemanticAnalysisError::SymbolRedefinition { name, .. } => Some(name.len()),
            SemanticAnalysisError::InvalidFieldType { field_name, .. } => Some(field_name.len()),
            SemanticAnalysisError::TypeMismatch { context, .. } => {
                context.as_ref().map(|s| s.len())
            }
            SemanticAnalysisError::OperationTypeMismatch { operator, .. } => Some(operator.len()),
            SemanticAnalysisError::LogicalOperatorTypeMismatch { operator, .. } => {
                Some(operator.len())
            }
            SemanticAnalysisError::ValueOutOfRange { value, .. } => Some(value.len()),
            SemanticAnalysisError::ArgumentCountMismatch { function_name, .. } => {
                Some(function_name.len())
            }
            SemanticAnalysisError::ArgumentTypeMismatch { function_name, .. } => {
                Some(function_name.len())
            }
            SemanticAnalysisError::ReturnOutsideFunction { .. } => Some("return".len()),
            SemanticAnalysisError::ReturnTypeMismatch { .. } => None,
            SemanticAnalysisError::MissingReturnValue { .. } => Some("return".len()),
            SemanticAnalysisError::UndefinedFunction { name, .. } => Some(name.len()),
            SemanticAnalysisError::InvalidUnaryOperation { operator, .. } => Some(operator.len()),
            SemanticAnalysisError::AssignmentToImmutableVariable { name, .. } => Some(name.len()),
            SemanticAnalysisError::InvalidExpression { .. } => None,
        }
    }

    /// Convert a SemanticAnalysisError to a CompilerError that can be used by the rest of the compiler.
    ///
    /// ### Arguments
    /// * `context` - The CompilationContext providing type names and other context for error messages
    ///
    /// ### Returns
    /// A CompilerError with the appropriate message and location information.
    pub fn to_compiler_error(&self, context: &CompilationContext) -> CompilerError {
        let location = self.get_location();
        let token_length = self.get_token_length();
        CompilerError::new(
            self.format_message(context),
            location.line,
            location.column,
            location.position,
            token_length,
        )
    }
}
