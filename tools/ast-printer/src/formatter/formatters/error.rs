use slang_error::{DomainError, CompilationError, ErrorCode, Location, ErrorCategory};

/// Custom error type for formatting operations
#[derive(Debug)]
pub struct FormatError {
    pub message: String,
}

impl FormatError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Format error: {}", self.message)
    }
}

impl std::error::Error for FormatError {}

impl DomainError for FormatError {
    fn to_compiler_error(&self) -> CompilationError {
        CompilationError::new(
            ErrorCode::InternalError,
            format!("Format error: {}", self.message),
            1, 1, 0, Some(1)
        )
    }
    
    fn location(&self) -> &Location {
        static DEFAULT_LOCATION: std::sync::LazyLock<Location> = std::sync::LazyLock::new(|| Location::new(0, 1, 1, 1));
        &DEFAULT_LOCATION
    }
    
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Codegen
    }
    
    fn short_description(&self) -> String {
        self.message.clone()
    }
}

impl From<FormatError> for Box<dyn DomainError> {
    fn from(error: FormatError) -> Self {
        Box::new(error)
    }
}
