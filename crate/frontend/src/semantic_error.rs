use crate::error::CompilerError;
use slang_types::types::{TypeId, get_type_name};

/// Represents different categories of semantic analysis errors
/// that occur during static analysis of the program.
/// 
/// Each variant contains the necessary context for generating
/// appropriate error messages that maintain the existing format.
#[derive(Debug)]
pub enum SemanticAnalysisError {
    /// An attempt to use a variable that has not been defined in scope
    UndefinedVariable {
        /// The name of the undefined variable
        name: String,
    },
    
    /// A variable with the same name is already defined in the current scope
    VariableRedefinition {
        /// The name of the variable being redefined
        name: String,
    },
    
    /// The type of an expression does not match the expected type
    TypeMismatch {
        /// The expected type
        expected: TypeId,
        /// The actual type found
        actual: TypeId,
        /// Optional context for the mismatch (like variable or function name)
        context: Option<String>,
    },
    
    /// Incompatible types for an operation like arithmetic or comparison
    OperationTypeMismatch {
        /// The operation being performed (e.g., +, -, *, /)
        operator: String,
        /// Left operand type
        left_type: TypeId,
        /// Right operand type
        right_type: TypeId,
    },
    
    /// Logical operators (AND, OR) used with non-boolean operands
    LogicalOperatorTypeMismatch {
        /// The logical operator being used (AND, OR)
        operator: String,
        /// Left operand type
        left_type: TypeId,
        /// Right operand type
        right_type: TypeId,
    },
    
    /// Value is out of range for the target type (e.g., integer overflow)
    ValueOutOfRange {
        /// The value that can't fit in the type
        value: String,
        /// The target type
        target_type: TypeId,
        /// Whether the value is an integer or float
        is_float: bool,
    },
    
    /// Function call with wrong number of arguments
    ArgumentCountMismatch {
        /// Function name
        function_name: String,
        /// Expected number of arguments
        expected: usize,
        /// Actual number of arguments provided
        actual: usize,
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
    },
    
    /// Return statement outside of a function
    ReturnOutsideFunction,
    
    /// Return type does not match function declaration
    ReturnTypeMismatch {
        /// Expected return type
        expected: TypeId,
        /// Actual returned type
        actual: TypeId,
    },
    
    /// Missing return value for a function that requires one
    MissingReturnValue {
        /// Expected return type
        expected: TypeId,
    },
    
    /// Undefined function in a function call
    UndefinedFunction {
        /// The name of the undefined function
        name: String,
    },
    
    /// Unary operation applied to incompatible type
    InvalidUnaryOperation {
        /// The unary operator (e.g., -, !)
        operator: String,
        /// The operand type
        operand_type: TypeId,
    },
    
    /// An expression has an unexpected form or context
    InvalidExpression {
        /// A description of what was expected vs what was found
        message: String,
    },
}

impl SemanticAnalysisError {
    /// Convert the SemanticAnalysisError to a String representation
    /// that matches the existing error message formats.
    pub fn to_string(&self) -> String {
        match self {
            SemanticAnalysisError::UndefinedVariable { name } => {
                format!("Undefined variable: {}", name)
            }
            
            SemanticAnalysisError::VariableRedefinition { name } => {
                format!("Variable '{}' already defined", name)
            }
            
            SemanticAnalysisError::TypeMismatch { expected, actual, context } => {
                if let Some(ctx) = context {
                    format!(
                        "Type mismatch: variable {} is {} but expression is {}",
                        ctx, get_type_name(expected), get_type_name(actual)
                    )
                } else {
                    format!(
                        "Type mismatch: expected {}, got {}",
                        get_type_name(expected), get_type_name(actual)
                    )
                }
            }
            
            SemanticAnalysisError::OperationTypeMismatch { operator, left_type, right_type } => {
                format!(
                    "Type mismatch: cannot apply '{}' operator on {} and {}",
                    operator, get_type_name(left_type), get_type_name(right_type)
                )
            }
            
            SemanticAnalysisError::LogicalOperatorTypeMismatch { operator, left_type, right_type } => {
                format!(
                    "Logical operator '{}' requires boolean operands, got {} and {}",
                    operator, get_type_name(left_type), get_type_name(right_type)
                )
            }
            
            SemanticAnalysisError::ValueOutOfRange { value, target_type, is_float } => {
                if *is_float {
                    format!(
                        "Float literal {} is out of range for type {}",
                        value, get_type_name(target_type)
                    )
                } else {
                    format!(
                        "Integer literal {} is out of range for type {}",
                        value, get_type_name(target_type)
                    )
                }
            }
            
            SemanticAnalysisError::ArgumentCountMismatch { function_name, expected, actual } => {
                format!(
                    "Function '{}' expects {} arguments, but got {}",
                    function_name, expected, actual
                )
            }
            
            SemanticAnalysisError::ArgumentTypeMismatch { function_name, argument_position, expected, actual } => {
                format!(
                    "Type mismatch: function '{}' expects argument {} to be {}, but got {}",
                    function_name, argument_position, get_type_name(expected), get_type_name(actual)
                )
            }
            
            SemanticAnalysisError::ReturnOutsideFunction => {
                "Return statement outside of function".to_string()
            }
            
            SemanticAnalysisError::ReturnTypeMismatch { expected, actual } => {
                format!(
                    "Type mismatch: function returns {} but got {}",
                    get_type_name(expected), get_type_name(actual)
                )
            }
            
            SemanticAnalysisError::MissingReturnValue { expected } => {
                format!(
                    "Type mismatch: function returns {} but no return value provided",
                    get_type_name(expected)
                )
            }
            
            SemanticAnalysisError::UndefinedFunction { name } => {
                format!("Undefined function: {}", name)
            }
            
            SemanticAnalysisError::InvalidUnaryOperation { operator, operand_type } => {
                if operator == "!" {
                    format!(
                        "Boolean not operator '!' can only be applied to boolean types, but got {}",
                        get_type_name(operand_type)
                    )
                } else if operator == "-" {
                    // Special handling for unsigned types to match existing error messages
                    if get_type_name(operand_type) == "u32" || get_type_name(operand_type) == "u64" {
                        "Cannot negate unsigned type".to_string()
                    } else {
                        format!(
                            "Cannot negate non-numeric type '{}'",
                            get_type_name(operand_type)
                        )
                    }
                } else {
                    format!(
                        "Cannot apply operator '{}' to type {}",
                        operator, get_type_name(operand_type)
                    )
                }
            },
            
            SemanticAnalysisError::InvalidExpression { message } => {
                message.clone()
            }
        }
    }
    
    /// Convert a SemanticAnalysisError to a CompilerError that can be used by the rest of the compiler.
    /// 
    /// ### Arguments
    /// * `line` - The line number where the error occurred
    /// * `column` - The column number where the error occurred
    /// 
    /// ### Returns
    /// A CompilerError with the appropriate message and location information.
    pub fn to_compiler_error(&self, line: usize, column: usize) -> CompilerError {
        CompilerError::new(self.to_string(), line, column)
    }
}
