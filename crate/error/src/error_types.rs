use std::fmt;
use crate::error_codes::ErrorCode;
use crate::compiler_error::CompilerError;
use slang_ir::location::Location;

#[derive(Debug, thiserror::Error)]
pub enum SlangError {
    #[error("Parse error: {0}")]
    Parse(ParseError),
    
    #[error("Semantic analysis error: {0}")]
    Semantic(SemanticError),
    
    #[error("Code generation error: {0}")]
    Codegen(CodegenError),
    
    #[error("Type system error: {0}")]
    Type(TypeError),
    
    #[error("IO error: {0}")]
    Io(String),
    
    #[error("Multiple compilation errors")]
    Multiple {
        errors: Vec<SlangError>,
        severity: ErrorSeverity,
    },
    
    #[error("Compiler error: {message}")]
    Compiler {
        error_code: ErrorCode,
        message: String,
        line: usize,
        column: usize,
        position: usize,
        token_length: Option<usize>,
    },
}

#[derive(Debug, Clone)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Note,
}

pub type SlangResult<T> = Result<T, SlangError>;

#[derive(Debug, Default)]
pub struct ErrorContext<'a> {
    pub file_name: Option<String>,
    pub source_text: Option<&'a str>,
    pub related_errors: Vec<String>, // Store error messages as strings instead of SlangError
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub error_code: ErrorCode,
    pub message: String,
    pub location: Location,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub error_code: ErrorCode,
    pub message: String,
    pub location: Location,
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SemanticError {}

#[derive(Debug, Clone)]
pub struct CodegenError {
    pub error_code: ErrorCode,
    pub message: String,
    pub location: Location,
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CodegenError {}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub error_code: ErrorCode,
    pub message: String,
    pub location: Location,
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TypeError {}

impl From<crate::compiler_error::CompilerError> for ParseError {
    fn from(error: crate::compiler_error::CompilerError) -> Self {
        Self {
            error_code: error.error_code,
            message: error.message,
            location: Location::new(error.position, error.line, error.column, error.token_length.unwrap_or(1)),
        }
    }
}

impl From<crate::compiler_error::CompilerError> for SemanticError {
    fn from(error: crate::compiler_error::CompilerError) -> Self {
        Self {
            error_code: error.error_code,
            message: error.message,
            location: Location::new(error.position, error.line, error.column, error.token_length.unwrap_or(1)),
        }
    }
}

impl From<crate::compiler_error::CompilerError> for CodegenError {
    fn from(error: crate::compiler_error::CompilerError) -> Self {
        Self {
            error_code: error.error_code,
            message: error.message,
            location: Location::new(error.position, error.line, error.column, error.token_length.unwrap_or(1)),
        }
    }
}

impl From<crate::compiler_error::CompilerError> for TypeError {
    fn from(error: crate::compiler_error::CompilerError) -> Self {
        Self {
            error_code: error.error_code,
            message: error.message,
            location: Location::new(error.position, error.line, error.column, error.token_length.unwrap_or(1)),
        }
    }
}

impl From<std::io::Error> for SlangError {
    fn from(error: std::io::Error) -> Self {
        SlangError::Io(error.to_string())
    }
}

impl From<CompilerError> for SlangError {
    fn from(error: CompilerError) -> Self {
        SlangError::Compiler {
            error_code: error.error_code,
            message: error.message,
            line: error.line,
            column: error.column,
            position: error.position,
            token_length: error.token_length,
        }
    }
}
