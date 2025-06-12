use slang_error::{CompilerError, ErrorCode, LineInfo};

/// Error that occurs during parsing
#[derive(Debug)]
pub struct ParseError {
    /// The structured error code for this error
    error_code: ErrorCode,
    /// Error message describing the problem
    message: String,
    /// Position in the source code where the error occurred
    position: usize,
    /// Length of the underlined part
    underline_length: usize,
}

impl ParseError {
    /// Creates a new parse error with the given error code, message and position
    pub fn new(
        error_code: ErrorCode,
        message: &str,
        position: usize,
        underline_length: usize,
    ) -> Self {
        ParseError {
            error_code,
            message: message.to_string(),
            position,
            underline_length,
        }
    }

    pub fn to_compiler_error(&self, line_info: &LineInfo) -> CompilerError {
        let line_pos = line_info.get_line_col(self.position);
        CompilerError::new(
            self.error_code,
            self.message.clone(),
            line_pos.0,
            line_pos.1,
            self.position,
            Some(self.underline_length),
        )
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

