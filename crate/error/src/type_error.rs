// Type system error types and implementations
use crate::compiler_error::CompilerError;
use crate::domain_error::{DomainError, ErrorCategory};
use crate::error_codes::ErrorCode;
use crate::Location;

/// Type system errors with detailed context
#[derive(Debug, Clone)]
pub enum TypeError {
    /// Type not found with suggestions
    TypeNotFound {
        name: String,
        suggestions: Vec<String>,
        location: Location,
    },
    /// Circular type definition
    CircularDefinition {
        name: String,
        dependency_chain: Vec<String>,
        location: Location,
    },
    /// Invalid type construction
    InvalidTypeConstruction {
        attempted_type: String,
        reason: String,
        location: Location,
    },
}

// Convenience result type for type operations
pub type TypeResult<T> = Result<T, TypeError>;

impl DomainError for TypeError {
    fn to_compiler_error(&self) -> CompilerError {
        let (code, message) = match self {
            TypeError::TypeNotFound { name, suggestions, .. } => {
                let base_msg = format!("Type not found: {}", name);
                let msg = if suggestions.is_empty() {
                    base_msg
                } else {
                    format!("{}. Did you mean: {}?", base_msg, suggestions.join(", "))
                };
                (ErrorCode::UndefinedType, msg)
            }
            TypeError::CircularDefinition { name, dependency_chain, .. } => {
                (ErrorCode::CircularDependency, 
                 format!("Circular type definition for '{}': {}", name, dependency_chain.join(" -> ")))
            }
            TypeError::InvalidTypeConstruction { attempted_type, reason, .. } => {
                (ErrorCode::InvalidType, 
                 format!("Invalid type construction for '{}': {}", attempted_type, reason))
            }
        };
        
        let loc = self.location();
        CompilerError::new(code, message, loc.line, loc.column, loc.position, Some(1))
    }
    
    fn location(&self) -> &Location {
        match self {
            TypeError::TypeNotFound { location, .. } |
            TypeError::CircularDefinition { location, .. } |
            TypeError::InvalidTypeConstruction { location, .. } => location,
        }
    }
    
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }
    
    fn short_description(&self) -> String {
        match self {
            TypeError::TypeNotFound { name, .. } => format!("Type not found: {}", name),
            TypeError::CircularDefinition { name, .. } => format!("Circular definition: {}", name),
            TypeError::InvalidTypeConstruction { attempted_type, .. } => format!("Invalid type: {}", attempted_type),
        }
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short_description())
    }
}

impl std::error::Error for TypeError {}
