// Semantic error types and implementations
use crate::compiler_error::CompilationError;
use crate::domain_error::{DomainError, ErrorCategory};
use crate::error_codes::ErrorCode;
use crate::Location;

/// Enhanced semantic analysis errors (based on existing SemanticAnalysisError)
#[derive(Debug, Clone)]
pub enum SemanticError {
    /// Variable/symbol not found with suggested alternatives
    UndefinedSymbol {
        name: String,
        suggestions: Vec<String>, // Similar names in scope
        location: Location,
    },
    /// Symbol redefinition with reference to original
    SymbolRedefinition {
        name: String,
        kind: String, // "variable", "function", "type"
        original_location: Location,
        redefinition_location: Location,
    },
    /// Type mismatch with conversion suggestions
    TypeMismatch {
        expected: String,
        found: String,
        context: String, // e.g., "in function argument 2"
        can_convert: bool, // Whether automatic conversion is possible
        location: Location,
    },
    /// Invalid operation for given types
    InvalidOperation {
        operation: String,
        left_type: String,
        right_type: String,
        suggestion: Option<String>, // e.g., "consider casting to common type"
        location: Location,
    },
    /// Function call errors
    FunctionCallError {
        function_name: String,
        error_kind: FunctionCallErrorKind,
        location: Location,
    },
}

#[derive(Debug, Clone)]
pub enum FunctionCallErrorKind {
    UndefinedFunction { suggestions: Vec<String> },
    ArgumentCountMismatch { expected: usize, found: usize },
    ArgumentTypeMismatch { 
        argument_index: usize,
        expected: String, 
        found: String,
        can_convert: bool,
    },
}

// Convenience result type for semantic operations
pub type SemanticResult<T> = Result<T, SemanticError>;

impl DomainError for SemanticError {
    fn to_compiler_error(&self) -> CompilationError {
        let (code, message) = match self {
            SemanticError::UndefinedSymbol { name, suggestions, .. } => {
                let base_msg = format!("Undefined symbol: {name}");
                let msg = if suggestions.is_empty() {
                    base_msg
                } else {
                    format!("{}. Did you mean: {}?", base_msg, suggestions.join(", "))
                };
                (ErrorCode::UndefinedVariable, msg)
            }
            SemanticError::SymbolRedefinition { name, kind, .. } => {
                (ErrorCode::SymbolRedefinition, 
                 format!("{kind} '{name}' is already defined in the current scope"))
            }
            SemanticError::TypeMismatch { expected, found, context: ctx, can_convert, .. } => {
                let base_msg = format!("Type mismatch: expected {}, found {} {}", 
                                     expected, found, 
                                     if ctx.is_empty() { String::new() } else { format!("({ctx})") });
                let msg = if *can_convert {
                    format!("{base_msg}. Consider explicit type conversion.")
                } else {
                    base_msg
                };
                (ErrorCode::TypeMismatch, msg)
            }
            SemanticError::InvalidOperation { operation, left_type, right_type, suggestion, .. } => {
                let base_msg = format!("Invalid operation '{operation}' between {left_type} and {right_type}");
                let msg = if let Some(suggestion) = suggestion {
                    format!("{base_msg}. {suggestion}")
                } else {
                    base_msg
                };
                (ErrorCode::OperationTypeMismatch, msg)
            }
            SemanticError::FunctionCallError { function_name, error_kind, .. } => {
                let msg = match error_kind {
                    FunctionCallErrorKind::UndefinedFunction { suggestions } => {
                        let base_msg = format!("Undefined function: {function_name}");
                        if suggestions.is_empty() {
                            base_msg
                        } else {
                            format!("{}. Did you mean: {}?", base_msg, suggestions.join(", "))
                        }
                    }
                    FunctionCallErrorKind::ArgumentCountMismatch { expected, found } => {
                        format!("Function '{function_name}' expects {expected} arguments, but {found} were provided")
                    }
                    FunctionCallErrorKind::ArgumentTypeMismatch { argument_index, expected, found, can_convert } => {
                        let base_msg = format!("Argument {} of function '{}': expected {}, found {}", 
                                             argument_index + 1, function_name, expected, found);
                        if *can_convert {
                            format!("{base_msg}. Consider explicit type conversion.")
                        } else {
                            base_msg
                        }
                    }
                };
                (ErrorCode::UndefinedFunction, msg)
            }
        };
        
        let loc = self.location();
        CompilationError::new(code, message, loc.line, loc.column, loc.position, Some(1))
    }
    
    fn location(&self) -> &Location {
        match self {
            SemanticError::UndefinedSymbol { location, .. } |
            SemanticError::TypeMismatch { location, .. } |
            SemanticError::InvalidOperation { location, .. } |
            SemanticError::FunctionCallError { location, .. } => location,
            SemanticError::SymbolRedefinition { redefinition_location, .. } => redefinition_location,
        }
    }
    
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Semantic
    }
    
    fn short_description(&self) -> String {
        match self {
            SemanticError::UndefinedSymbol { name, .. } => format!("Undefined symbol: {name}"),
            SemanticError::SymbolRedefinition { name, .. } => format!("Symbol redefinition: {name}"),
            SemanticError::TypeMismatch { .. } => "Type mismatch".to_string(),
            SemanticError::InvalidOperation { operation, .. } => format!("Invalid operation: {operation}"),
            SemanticError::FunctionCallError { function_name, .. } => format!("Function call error: {function_name}"),
        }
    }
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short_description())
    }
}

impl std::error::Error for SemanticError {}
