use colored::Colorize;

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub position: Option<usize>,
    pub length: Option<usize>,
    pub formatted_message: Option<String>,
    pub source_snippet: Option<String>,
}

impl CompilerError {
    pub fn new(message: String, line: usize, column: usize) -> Self {
        Self {
            message,
            line,
            column,
            position: None,
            length: None,
            formatted_message: None,
            source_snippet: None,
        }
    }
    
    pub fn with_position(message: String, line: usize, column: usize, position: usize, length: usize) -> Self {
        Self {
            message,
            line,
            column,
            position: Some(position),
            length: Some(length),
            formatted_message: None,
            source_snippet: None,
        }
    }
    
    pub fn with_formatted_message(mut self, formatted_message: String) -> Self {
        self.formatted_message = Some(formatted_message);
        self
    }

    pub fn with_source_snippet(mut self, source_snippet: String) -> Self {
        self.source_snippet = Some(source_snippet);
        self
    }
    
    /// Creates a compiler error from line information
    /// 
    /// # Arguments
    /// * `line_info` - Line information containing source code
    /// * `message` - Error message
    /// * `position` - Position of the error in source code
    /// * `length` - Length of the token with the error
    /// 
    /// # Returns
    /// A CompilerError with source context information
    pub fn from_line_info(line_info: &LineInfo, message: String, position: usize, length: usize) -> Self {
        let (line, column) = line_info.get_line_col(position);
        let formatted_error = line_info.format_error(position, &message, length);
        
        Self {
            message,
            line,
            column,
            position: Some(position),
            length: Some(length),
            formatted_message: Some(formatted_error),
            source_snippet: line_info.get_line_text(line).map(String::from),
        }
    }

    pub fn format(&self) -> String {
        if let Some(formatted) = &self.formatted_message {
            return formatted.clone();
        }
        
        // Format the line number as a string to determine indentation
        let line_string = format!("{}", self.line);
        
        // Create indentation with 1 more whitespace than the line string length
        let indent = " ".repeat(line_string.len() + 1);
        let pipe = "|".yellow(); // Pipe character for formatting
        let arrow = "-->".yellow();
        // Create error marker with caret depending on if we know the token length
        let mut error_display = format!(
            "  {} line {}:{}\n{indent}{}\n",
            arrow, self.line, self.column, pipe
        );
        
        // Add source code snippet if available
        if let Some(snippet) = &self.source_snippet {
            error_display += &format!("{} {} {}\n", pipe, self.line, snippet);
            
            if let (Some(_), Some(len)) = (self.position, self.length) {
                // If we have position and length, show error marker with carets
                let marker = " ".repeat(self.column - 1) + &"^".repeat(len.max(1)).red().to_string();
                error_display += &format!("{indent}{pipe} {}\n", marker);
            }
        }
        
        // Add the error message
        error_display += &format!("{}", self.message);
        
        error_display
    }
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

pub type CompileResult<T> = Result<T, Vec<CompilerError>>;

/// Reports a list of compiler errors to stderr
pub fn report_errors(errors: &[CompilerError]) {
    for error in errors.iter() {
        eprintln!("{}", error);
    }
}

pub struct ErrorCollector {
    errors: Vec<CompilerError>,
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
            eprintln!("{}", error.format());
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
    /// # Arguments
    /// * `pos` - The position of the token in the source code
    /// 
    /// # Returns
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
        
        let actual_end = if start < end && end > 0 && 
                           self.source.as_bytes().get(end - 1) == Some(&b'\n') {
            end - 1
        } else {
            end
        };
        
        Some(&self.source[start..actual_end])
    }
    
    /// Format an error message with line information and source code snippet
    /// 
    /// This creates a nicely formatted error message similar to Rust's compiler errors,
    /// with line numbers, source code context, and arrows pointing to the error location.
    ///
    /// # Arguments
    /// * `pos` - The position of the error in the source code
    /// * `message` - The error message
    /// * `token_length` - The length of the problematic token
    ///
    /// # Returns
    /// A formatted error message string
    pub fn format_error(&self, pos: usize, message: &str, token_length: usize) -> String {
        let (line, col) = self.get_line_col(pos);
        
        // Get line text for current line and previous line if available
        let current_line_text = self.get_line_text(line).unwrap_or("<line not available>");
      
        // Format line numbers for consistent spacing
        let line_num = format!("{}", line);
        
        // Create indentation with 1 more whitespace than the line string length
        let indent = " ".repeat(line_num.len() + 1);
        
        // Create the error marker line with red carets
        let error_marker = " ".repeat(col - 1) + &"^".repeat(token_length.max(1)).red().to_string();
        
        // Build the nicely formatted error message
        let pipe = "|".yellow(); 
        let mut result = format!("{} {} {}\n", line_num.yellow(), pipe, current_line_text);
        result += &format!("{indent}{} {}\n", pipe, error_marker);
        result += &format!("{indent}{} {}\n",pipe,  message);
        
        result
    }
}
