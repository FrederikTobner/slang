use crate::error_codes::ErrorCode;
use colored::Colorize;

/// Represents a compiler error with a message, line number, column number, position, and token length
#[derive(Debug)]
pub struct CompilerError {
    /// The structured error code for this error
    pub error_code: ErrorCode,
    /// The error message
    pub message: String,
    /// The line number where the error occurred
    pub line: usize,
    /// The column number where the error occurred
    pub column: usize,
    /// The byte offset position of the error in the source code
    pub position: usize,
    /// The length of the token causing the error, if applicable
    pub token_length: Option<usize>,
}

impl CompilerError {
    /// Creates a new CompilerError with the given error code, message, line number, column number, position, and token length
    ///
    /// ### Arguments
    /// * `error_code` - The structured error code for this error
    /// * `message` - The error message
    /// * `line` - The line number where the error occurred
    /// * `column` - The column number where the error occurred
    /// * `position` - The byte offset position of the error
    /// * `token_length` - The length of the token, if applicable
    ///
    /// ### Returns
    /// A new CompilerError object
    ///
    /// ### Example
    /// ```
    /// use slang_frontend::error::CompilerError;
    /// use slang_frontend::error_codes::ErrorCode;
    ///
    /// let error = CompilerError::new(ErrorCode::ExpectedSemicolon, "Syntax error".to_string(), 10, 5, 0, Some(1));
    /// ```
    pub fn new(
        error_code: ErrorCode,
        message: String,
        line: usize,
        column: usize,
        position: usize,
        token_length: Option<usize>,
    ) -> Self {
        Self {
            error_code,
            message,
            line,
            column,
            position,
            token_length,
        }
    }

    /// Format an error message with line information and source code snippet
    ///
    /// This creates a nicely formatted error message similar to Rust's compiler errors,
    /// with line numbers, source code context, and arrows pointing to the error location.
    ///
    /// ### Arguments
    /// * `line_info` - LineInfo object for the source code context
    ///
    /// ### Returns
    /// A formatted error message string
    pub fn format_for_display(&self, line_info: &LineInfo) -> String {
        let (line, col) = line_info.get_line_col(self.position);

        let current_line_text = line_info
            .get_line_text(line)
            .unwrap_or("<line not available>");

        let line_num_str = format!("{}", line);

        let token_display_length = self.token_length.unwrap_or(1).max(1);
        let error_marker = " ".repeat(col.saturating_sub(1))
            + &"^".repeat(token_display_length).bold().red().to_string();

        let indent_width = line_num_str.len() + 1;
        let indent = " ".repeat(indent_width);

        let arrow = "-->".yellow();
        let pipe = "|".yellow();

        let mut result = format!(
            "{} {}: {}\n  {} {}:{}:{}\n",
            "error".red().bold(),
            self.error_code.to_string().bold().red(),
            self.error_code.description(),
            arrow,
            "main", // TODO: replace if actual filename is available
            line,
            col
        );

        result += &format!("{indent}{}\n", pipe);
        result += &format!("{} {} {}\n", line_num_str.yellow(), pipe, current_line_text);
        result += &format!(
            "{indent}{} {} {}\n",
            pipe,
            error_marker,
            self.message.bold().red()
        );

        result
    }
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// A type alias for a result that can either be a value of type T or a list of compiler errors
pub type CompileResult<T> = Result<T, Vec<CompilerError>>;

/// Reports a list of compiler errors to stderr
///
/// ### Arguments
/// * `errors` - A slice of CompilerError to report
/// * `source` - The source code string, used for generating line information
pub fn report_errors(errors: &[CompilerError], source: &str) {
    let line_info = LineInfo::new(source);
    for error in errors.iter() {
        eprintln!("{}", error.format_for_display(&line_info));
    }
}

pub struct ErrorCollector {
    errors: Vec<CompilerError>,
}

impl Default for ErrorCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn report_errors(&self) {
        for error in self.errors.iter() {
            eprintln!("{}", error.message);
        }
    }

    pub fn clear(&mut self) {
        self.errors.clear();
    }

    pub fn take_errors(&mut self) -> Vec<CompilerError> {
        std::mem::take(&mut self.errors)
    }
}

pub struct LineInfo<'a> {
    /// Number of tokens on each line (run-length encoded)
    /// (line_number, tokens_on_line)
    pub per_line: Vec<(u16, u16)>,
    /// Reference to the original source code
    source: &'a str,
    /// The starting position of each line in the source code
    pub line_starts: Vec<usize>,
}

impl LineInfo<'_> {
    /// Creates a new LineInfo object
    ///
    /// ### Arguments
    /// * `source` - The source code string
    ///
    /// ### Returns
    /// A new LineInfo object with the line starts calculated
    pub fn new(source: &str) -> LineInfo {
        let mut line_starts = vec![0];

        for (i, c) in source.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }

        LineInfo {
            per_line: Vec::new(),
            source,
            line_starts,
        }
    }

    /// Get the line and column number for a token position
    ///
    /// ### Arguments
    /// * `pos` - The position of the token in the source code
    ///
    /// ### Returns
    /// A tuple containing the line number and column number
    pub fn get_line_col(&self, pos: usize) -> (usize, usize) {
        match self.line_starts.binary_search(&pos) {
            Ok(line) => (line + 1, 1),
            Err(line) => {
                let line_idx = line - 1;
                let col = pos - self.line_starts[line_idx] + 1;
                (line_idx + 1, col)
            }
        }
    }

    /// Get the text for a specific line
    ///
    /// ### Arguments
    /// * `line` - The line number to retrieve
    ///
    /// ### Returns
    /// The text of the line, or None if the line number is invalid
    pub fn get_line_text(&self, line: usize) -> Option<&str> {
        if line == 0 || line > self.line_starts.len() {
            return None;
        }

        let start = self.line_starts[line - 1];
        let end = if line < self.line_starts.len() {
            self.line_starts[line]
        } else {
            self.source.len()
        };

        let actual_end =
            if start < end && end > 0 && self.source.as_bytes().get(end - 1) == Some(&b'\n') {
                end - 1
            } else {
                end
            };

        Some(&self.source[start..actual_end])
    }
}
