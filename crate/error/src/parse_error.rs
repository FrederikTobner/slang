// Parse error types and implementations
use crate::Location;
use crate::compiler_error::CompilationError;
use crate::domain_error::{DomainError, ErrorCategory};
use crate::error_codes::ErrorCode;

/// Enhanced parsing errors with richer context than current ParseError
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Expected a specific token but found something else
    ExpectedToken {
        expected: String,
        found: String,
        location: Location,
        context: String,               // e.g., "in function parameter list"
        error_code: Option<ErrorCode>, // Optional specific error code for backward compatibility
    },
    /// Unexpected end of file
    UnexpectedEof {
        expected: String,
        location: Location,
        context: String,
        error_code: Option<ErrorCode>,
    },
    /// Invalid numeric literal with detailed reason
    InvalidNumber {
        value: String,
        reason: String, // e.g., "value exceeds u64 maximum"
        location: Location,
        error_code: Option<ErrorCode>,
    },
    /// Invalid syntax construct with recovery suggestion
    InvalidSyntax {
        message: String,
        suggestion: Option<String>, // e.g., "try adding a semicolon"
        location: Location,
        error_code: Option<ErrorCode>,
    },
    /// Mismatched delimiters (brackets, braces, parentheses)
    MismatchedDelimiters {
        opening: String,
        expected_closing: String,
        found_closing: Option<String>,
        location: Location,
        error_code: Option<ErrorCode>,
    },
}

// Convenience result type for parse operations
pub type ParseResult<T> = Result<T, ParseError>;

// Implement DomainError for ParseError
impl DomainError for ParseError {
    fn to_compiler_error(&self) -> CompilationError {
        let (code, message) = match self {
            ParseError::ExpectedToken {
                expected,
                found,
                context,
                error_code,
                ..
            } => {
                let final_code = error_code.unwrap_or(ErrorCode::ExpectedToken);
                (
                    final_code,
                    format!(
                        "Expected {}, found {} {}",
                        expected,
                        found,
                        if context.is_empty() {
                            String::new()
                        } else {
                            format!("({context})")
                        }
                    ),
                )
            }
            ParseError::UnexpectedEof {
                expected,
                context,
                error_code,
                ..
            } => {
                let final_code = error_code.unwrap_or(ErrorCode::UnexpectedEof);
                (
                    final_code,
                    format!(
                        "Unexpected end of file, expected {} {}",
                        expected,
                        if context.is_empty() {
                            String::new()
                        } else {
                            format!("({context})")
                        }
                    ),
                )
            }
            ParseError::InvalidNumber {
                value,
                reason,
                error_code,
                ..
            } => {
                let final_code = error_code.unwrap_or(ErrorCode::InvalidNumber);
                (final_code, format!("Invalid number '{value}': {reason}"))
            }
            ParseError::InvalidSyntax {
                message,
                suggestion,
                error_code,
                ..
            } => {
                let final_code = error_code.unwrap_or(ErrorCode::SyntaxError);
                (
                    final_code,
                    if let Some(suggestion) = suggestion {
                        format!("{message}. {suggestion}")
                    } else {
                        message.clone()
                    },
                )
            }
            ParseError::MismatchedDelimiters {
                opening,
                expected_closing,
                found_closing,
                error_code,
                ..
            } => {
                let final_code = error_code.unwrap_or(ErrorCode::MismatchedDelimiters);
                (
                    final_code,
                    match found_closing {
                        Some(found) => format!(
                            "Mismatched delimiters: opened with '{opening}', expected '{expected_closing}', found '{found}'"
                        ),
                        None => format!(
                            "Unclosed delimiter: '{opening}', expected '{expected_closing}'"
                        ),
                    },
                )
            }
        };

        let loc = self.location();
        CompilationError::new(code, message, loc.line, loc.column, loc.position, Some(1))
    }

    fn location(&self) -> &Location {
        match self {
            ParseError::ExpectedToken { location, .. }
            | ParseError::UnexpectedEof { location, .. }
            | ParseError::InvalidNumber { location, .. }
            | ParseError::InvalidSyntax { location, .. }
            | ParseError::MismatchedDelimiters { location, .. } => location,
        }
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Syntax
    }

    fn short_description(&self) -> String {
        match self {
            ParseError::ExpectedToken { expected, .. } => format!("Expected {expected}"),
            ParseError::UnexpectedEof { .. } => "Unexpected EOF".to_string(),
            ParseError::InvalidNumber { .. } => "Invalid number".to_string(),
            ParseError::InvalidSyntax { .. } => "Syntax error".to_string(),
            ParseError::MismatchedDelimiters { .. } => "Mismatched delimiters".to_string(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short_description())
    }
}

impl std::error::Error for ParseError {}
