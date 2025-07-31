// Core trait and types for the unified error system
use crate::compiler_error::CompilerError;
use slang_ir::Location;

/// Core trait for all domain-specific errors that can be converted to CompilerError
/// 
/// This trait allows each domain to maintain rich, specific error types while providing
/// a consistent interface for conversion to the unified CompilerError for reporting.
/// This preserves the existing error handling patterns while enabling better composability.
pub trait DomainError: std::error::Error + Send + Sync + 'static {
    /// Convert this domain error to a CompilerError for unified reporting
    /// 
    /// Each domain implements this method to provide context-aware conversion
    /// that preserves as much information as possible in the final error message.
    /// The context parameter is passed from the domain that has access to it.
    fn to_compiler_error(&self) -> CompilerError;
    
    /// Get the source location where this error occurred
    fn location(&self) -> &Location;
    
    /// Get the error category for filtering and organization
    fn category(&self) -> ErrorCategory;
    
    /// Get a short description of the error for logging/debugging
    fn short_description(&self) -> String;
}

/// Categories for organizing errors by domain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Lexical,
    Syntax,
    Semantic,
    Type,
    Codegen,
    IO,
}

// Standard result type using trait objects for maximum flexibility
pub type DomainResult<T> = Result<T, Box<dyn DomainError>>;
