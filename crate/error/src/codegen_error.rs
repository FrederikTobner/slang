// Code generation error types and implementations
use crate::compiler_error::CompilerError;
use crate::domain_error::{DomainError, ErrorCategory};
use crate::error_codes::ErrorCode;
use slang_ir::Location;

/// Code generation errors with actionable information
#[derive(Debug, Clone)]
pub enum CodegenError {
    /// Stack overflow during compilation
    StackOverflow {
        current_depth: usize,
        max_depth: usize,
        location: Location,
    },
    /// Too many constants/locals/etc.
    LimitExceeded {
        resource: String, // "constants", "local variables", etc.
        current: usize,
        limit: usize,
        location: Location,
    },
    /// Unsupported language feature
    UnsupportedFeature {
        feature: String,
        reason: String, // Why it's not supported
        alternative: Option<String>, // Suggested workaround
        location: Location,
    },
    /// Internal compiler error (should not happen in normal use)
    InternalError {
        message: String,
        location: Location,
    },
}

// Convenience result type for codegen operations
pub type CodegenResult<T> = Result<T, CodegenError>;

impl DomainError for CodegenError {
    fn to_compiler_error(&self) -> CompilerError {
        let (code, message) = match self {
            CodegenError::StackOverflow { current_depth, max_depth, .. } => {
                (ErrorCode::StackOverflow, 
                 format!("Stack overflow during compilation: depth {} exceeds maximum {}", 
                        current_depth, max_depth))
            }
            CodegenError::LimitExceeded { resource, current, limit, .. } => {
                (ErrorCode::TooManyConstants, 
                 format!("Too many {}: {} exceeds limit of {}", resource, current, limit))
            }
            CodegenError::UnsupportedFeature { feature, reason, alternative, .. } => {
                let base_msg = format!("Unsupported feature '{}': {}", feature, reason);
                let msg = if let Some(alternative) = alternative {
                    format!("{}. Try: {}", base_msg, alternative)
                } else {
                    base_msg
                };
                (ErrorCode::UnsupportedFeature, msg)
            }
            CodegenError::InternalError { message, .. } => {
                (ErrorCode::InternalError, format!("Internal compiler error: {}", message))
            }
        };
        
        let loc = self.location();
        CompilerError::new(code, message, loc.line, loc.column, loc.position, Some(1))
    }
    
    fn location(&self) -> &Location {
        match self {
            CodegenError::StackOverflow { location, .. } |
            CodegenError::LimitExceeded { location, .. } |
            CodegenError::UnsupportedFeature { location, .. } |
            CodegenError::InternalError { location, .. } => location,
        }
    }
    
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Codegen
    }
    
    fn short_description(&self) -> String {
        match self {
            CodegenError::StackOverflow { .. } => "Stack overflow".to_string(),
            CodegenError::LimitExceeded { resource, .. } => format!("Limit exceeded: {}", resource),
            CodegenError::UnsupportedFeature { feature, .. } => format!("Unsupported feature: {}", feature),
            CodegenError::InternalError { .. } => "Internal error".to_string(),
        }
    }
}

impl std::fmt::Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short_description())
    }
}

impl std::error::Error for CodegenError {}
