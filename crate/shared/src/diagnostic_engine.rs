use slang_error::{ErrorCode, LineInfo, CompilerError};
use slang_ir::location::Location;
use colored::Colorize;

/// Represents the severity level of a diagnostic message
#[derive(Debug, Clone)]
pub enum ErrorSeverity {
    /// A compilation error that prevents successful compilation
    Error,
    /// A warning that doesn't prevent compilation but may indicate issues
    Warning,
    /// An informational note providing additional context
    Note,
}

/// Represents a single diagnostic message with context and suggestions
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// The severity level of this diagnostic
    pub severity: ErrorSeverity,
    /// The structured error code for this diagnostic
    pub error_code: ErrorCode,
    /// The human-readable message describing the issue
    pub message: String,
    /// The source location where this diagnostic occurred
    pub location: Location,
    /// Optional suggestions for fixing the issue
    pub suggestions: Vec<Suggestion>,
    /// Related diagnostics that provide additional context
    pub related: Vec<Diagnostic>,
}

/// Represents a suggestion for fixing a diagnostic issue
#[derive(Debug, Clone)]
pub struct Suggestion {
    /// The suggestion message explaining how to fix the issue
    pub message: String,
    /// Optional replacement text for the problematic code
    pub replacement: Option<String>,
    /// Optional location where the replacement should be applied
    pub location: Option<Location>,
}

/// A diagnostic collection and reporting engine for compiler errors, warnings, and notes
///
/// The DiagnosticEngine serves as the central hub for collecting, managing, and reporting
/// all kinds of diagnostic messages during compilation. It supports error recovery mode,
/// rich formatting with source code context, and can emit diagnostics in various formats.
///
/// ### Features
/// - Collects errors, warnings, and notes with source location information
/// - Supports error recovery mode for collecting multiple errors in one pass
/// - Rich formatting with colored output and source code context
/// - Configurable error limits to prevent overwhelming output
/// - Integration with CompilerError for unified error handling
///
/// ### Example
/// ```rust
/// use slang_shared::DiagnosticEngine;
/// use slang_error::ErrorCode;
/// use slang_ir::location::Location;
///
/// let mut engine = DiagnosticEngine::new();
/// engine.set_file_name("example.sl".to_string());
/// engine.emit_error(
///     ErrorCode::ExpectedSemicolon,
///     "Missing semicolon".to_string(),
///     Location::new(42, 5, 10, 1)
/// );
/// 
/// if engine.has_errors() {
///     engine.report_all(&source_code);
/// }
/// ```
pub struct DiagnosticEngine<'a> {
    diagnostics: Vec<Diagnostic>,
    error_count: usize,
    warning_count: usize,
    max_errors: usize,
    recovery_mode: bool,
    file_name: Option<String>,
    source_text: Option<&'a str>,
}

impl<'a> DiagnosticEngine<'a> {
    /// Creates a new diagnostic engine with default settings
    ///
    /// ### Returns
    /// A new DiagnosticEngine instance ready for collecting diagnostics
    ///
    /// ### Example
    /// ```rust
    /// use slang_shared::DiagnosticEngine;
    /// 
    /// let engine = DiagnosticEngine::new();
    /// ```
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            error_count: 0,
            warning_count: 0,
            max_errors: 100,
            recovery_mode: false,
            file_name: None,
            source_text: None,
        }
    }
    
    /// Emits a diagnostic message to the engine
    ///
    /// This is the core method for adding diagnostics. It handles error counting,
    /// enforces error limits, and manages the diagnostic collection.
    ///
    /// ### Arguments
    /// * `diagnostic` - The diagnostic to emit
    ///
    /// ### Example
    /// ```rust
    /// use slang_shared::{DiagnosticEngine, Diagnostic, ErrorSeverity};
    /// use slang_error::ErrorCode;
    /// use slang_ir::location::Location;
    /// 
    /// let mut engine = DiagnosticEngine::new();
    /// let diagnostic = Diagnostic {
    ///     severity: ErrorSeverity::Error,
    ///     error_code: ErrorCode::ExpectedSemicolon,
    ///     message: "Missing semicolon".to_string(),
    ///     location: Location::new(42, 5, 10, 1),
    ///     suggestions: Vec::new(),
    ///     related: Vec::new(),
    /// };
    /// engine.emit(diagnostic);
    /// ```
    pub fn emit(&mut self, diagnostic: Diagnostic) {
        match diagnostic.severity {
            ErrorSeverity::Error => {
                self.error_count += 1;
                if self.error_count >= self.max_errors {
                    self.emit_too_many_errors();
                    return;
                }
            }
            ErrorSeverity::Warning => self.warning_count += 1,
            ErrorSeverity::Note => {}
        }
        self.diagnostics.push(diagnostic);
    }
    
    /// Emits an error diagnostic with the specified details
    ///
    /// This is a convenience method for creating and emitting error diagnostics.
    ///
    /// ### Arguments
    /// * `error_code` - The structured error code
    /// * `message` - The error message
    /// * `location` - The source location where the error occurred
    ///
    /// ### Example
    /// ```rust
    /// use slang_shared::DiagnosticEngine;
    /// use slang_error::ErrorCode;
    /// use slang_ir::location::Location;
    /// 
    /// let mut engine = DiagnosticEngine::new();
    /// engine.emit_error(
    ///     ErrorCode::ExpectedSemicolon,
    ///     "Missing semicolon after statement".to_string(),
    ///     Location::new(42, 5, 10, 1)
    /// );
    /// ```
    pub fn emit_error(&mut self, error_code: ErrorCode, message: String, location: Location) {
        self.emit(Diagnostic {
            severity: ErrorSeverity::Error,
            error_code,
            message,
            location,
            suggestions: Vec::new(),
            related: Vec::new(),
        });
    }
    
    /// Emits a warning diagnostic with the specified details
    ///
    /// This is a convenience method for creating and emitting warning diagnostics.
    ///
    /// ### Arguments
    /// * `error_code` - The structured error code
    /// * `message` - The warning message
    /// * `location` - The source location where the warning occurred
    ///
    /// ### Example
    /// ```rust
    /// use slang_shared::DiagnosticEngine;
    /// use slang_error::ErrorCode;
    /// use slang_ir::location::Location;
    /// 
    /// let mut engine = DiagnosticEngine::new();
    /// engine.emit_warning(
    ///     ErrorCode::UnusedVariable,
    ///     "Variable 'x' is declared but never used".to_string(),
    ///     Location::new(15, 3, 5, 1)
    /// );
    /// ```
    pub fn emit_warning(&mut self, error_code: ErrorCode, message: String, location: Location) {
        self.emit(Diagnostic {
            severity: ErrorSeverity::Warning,
            error_code,
            message,
            location,
            suggestions: Vec::new(),
            related: Vec::new(),
        });
    }
    
    /// Emits an error diagnostic with a suggestion for fixing the issue
    ///
    /// This is useful for providing actionable feedback to users about how to fix errors.
    ///
    /// ### Arguments
    /// * `error_code` - The structured error code
    /// * `message` - The error message
    /// * `location` - The source location where the error occurred
    /// * `suggestion` - A suggestion for fixing the error
    ///
    /// ### Example
    /// ```rust
    /// use slang_shared::{DiagnosticEngine, Suggestion};
    /// use slang_error::ErrorCode;
    /// use slang_ir::location::Location;
    /// 
    /// let mut engine = DiagnosticEngine::new();
    /// let suggestion = Suggestion {
    ///     message: "Add a semicolon".to_string(),
    ///     replacement: Some(";".to_string()),
    ///     location: Some(Location::new(42, 5, 10, 0)),
    /// };
    /// engine.emit_with_suggestion(
    ///     ErrorCode::ExpectedSemicolon,
    ///     "Missing semicolon".to_string(),
    ///     Location::new(42, 5, 10, 1),
    ///     suggestion
    /// );
    /// ```
    pub fn emit_with_suggestion(&mut self, error_code: ErrorCode, message: String, 
                                location: Location, suggestion: Suggestion) {
        self.emit(Diagnostic {
            severity: ErrorSeverity::Error,
            error_code,
            message,
            location,
            suggestions: vec![suggestion],
            related: Vec::new(),
        });
    }
    
    /// Directly emits a CompilerError as a diagnostic
    ///
    /// This method provides seamless integration with the existing CompilerError type,
    /// allowing for unified error handling across the compiler pipeline.
    ///
    /// ### Arguments
    /// * `error` - The CompilerError to emit as a diagnostic
    ///
    /// ### Example
    /// ```rust
    /// use slang_shared::DiagnosticEngine;
    /// use slang_error::{CompilerError, ErrorCode};
    /// 
    /// let mut engine = DiagnosticEngine::new();
    /// let error = CompilerError::new(
    ///     ErrorCode::ExpectedSemicolon,
    ///     "Missing semicolon".to_string(),
    ///     5, 10, 42, Some(1)
    /// );
    /// engine.emit_compiler_error(error);
    /// ```
    pub fn emit_compiler_error(&mut self, error: CompilerError) {
        let diagnostic = Diagnostic {
            severity: ErrorSeverity::Error,
            error_code: error.error_code,
            message: error.message.clone(),
            location: Location::new(error.position, error.line, error.column, error.token_length.unwrap_or(1)),
            suggestions: Vec::new(),
            related: Vec::new(),
        };
        self.emit(diagnostic);
    }
    
    /// Retrieves all error diagnostics as CompilerError instances
    ///
    /// This method converts all error-level diagnostics back to CompilerError format,
    /// useful for interfacing with code that expects the traditional CompilerError type.
    ///
    /// ### Returns
    /// A vector of CompilerError instances representing all errors collected
    ///
    /// ### Example
    /// ```rust
    /// use slang_shared::DiagnosticEngine;
    /// use slang_error::ErrorCode;
    /// use slang_ir::location::Location;
    /// 
    /// let mut engine = DiagnosticEngine::new();
    /// engine.emit_error(
    ///     ErrorCode::ExpectedSemicolon,
    ///     "Missing semicolon".to_string(),
    ///     Location::new(42, 5, 10, 1)
    /// );
    /// 
    /// let errors = engine.get_compiler_errors();
    /// assert_eq!(errors.len(), 1);
    /// ```
    pub fn get_compiler_errors(&self) -> Vec<CompilerError> {
        self.diagnostics.iter()
            .filter(|d| matches!(d.severity, ErrorSeverity::Error))
            .map(|d| CompilerError::new(
                d.error_code,
                d.message.clone(),
                d.location.line,
                d.location.column,
                d.location.position,
                Some(d.location.length),
            ))
            .collect()
    }
    
    /// Checks if any errors have been collected
    ///
    /// ### Returns
    /// `true` if there are any error-level diagnostics, `false` otherwise
    ///
    /// ### Example
    /// ```rust
    /// use slang_shared::DiagnosticEngine;
    /// 
    /// let mut engine = DiagnosticEngine::new();
    /// assert!(!engine.has_errors());
    /// 
    /// // After emitting an error...
    /// // assert!(engine.has_errors());
    /// ```
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }
    
    /// Returns the total number of errors collected
    ///
    /// ### Returns
    /// The count of error-level diagnostics
    pub fn error_count(&self) -> usize {
        self.error_count
    }
    
    /// Returns the total number of warnings collected
    ///
    /// ### Returns
    /// The count of warning-level diagnostics
    pub fn warning_count(&self) -> usize {
        self.warning_count
    }
    
    /// Finishes diagnostic collection and returns the result
    ///
    /// ### Returns
    /// `Ok(())` if no errors were collected, otherwise `Err` with all diagnostics
    pub fn finish(self) -> Result<(), Vec<Diagnostic>> {
        if self.has_errors() {
            Err(self.diagnostics)
        } else {
            Ok(())
        }
    }
    
    /// Enables or disables error recovery mode
    ///
    /// In recovery mode, the compilation pipeline continues processing even after
    /// encountering errors, allowing multiple errors to be collected in a single pass.
    ///
    /// ### Arguments
    /// * `enabled` - Whether to enable recovery mode
    pub fn set_recovery_mode(&mut self, enabled: bool) {
        self.recovery_mode = enabled;
    }
    
    /// Checks if error recovery mode is enabled
    ///
    /// ### Returns
    /// `true` if recovery mode is enabled, `false` otherwise
    pub fn is_recovery_mode(&self) -> bool {
        self.recovery_mode
    }
    
    /// Sets the file name for better error reporting
    ///
    /// ### Arguments
    /// * `file_name` - The name of the file being compiled
    pub fn set_file_name(&mut self, file_name: String) {
        self.file_name = Some(file_name);
    }
    
    /// Sets the source text for better error reporting
    ///
    /// ### Arguments
    /// * `source_text` - The source code being compiled
    pub fn set_source_text(&mut self, source_text: &'a str) {
        self.source_text = Some(source_text);
    }
    
    /// Sets the maximum number of errors before stopping compilation
    ///
    /// ### Arguments
    /// * `max_errors` - The maximum number of errors to collect
    pub fn set_max_errors(&mut self, max_errors: usize) {
        self.max_errors = max_errors;
    }
    
    /// Consumes the engine and returns all collected diagnostics
    ///
    /// ### Returns
    /// A vector containing all diagnostics that were collected
    pub fn into_errors(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
    
    /// Reports all diagnostics to stderr with rich formatting
    ///
    /// This method provides comprehensive error reporting with colored output,
    /// source code context, line numbers, and helpful suggestions.
    ///
    /// ### Arguments
    /// * `source` - The source code to display in error context
    ///
    /// ### Example
    /// ```rust
    /// use slang_shared::DiagnosticEngine;
    /// 
    /// let engine = DiagnosticEngine::new();
    /// // ... collect some diagnostics ...
    /// engine.report_all(&source_code);
    /// ```
    pub fn report_all(&self, source: &str) {
        let line_info = LineInfo::new(source);
        for diagnostic in &self.diagnostics {
            self.report_diagnostic(diagnostic, &line_info);
        }
        
        if self.error_count > 0 || self.warning_count > 0 {
            self.report_summary();
        }
    }
    
    /// Emits a "too many errors" diagnostic when the error limit is reached
    ///
    /// This private method is called automatically when the error count reaches
    /// the configured maximum, helping to prevent overwhelming output in cases
    /// of cascading errors.
    fn emit_too_many_errors(&mut self) {
        let diagnostic = Diagnostic {
            severity: ErrorSeverity::Error,
            error_code: ErrorCode::GenericCompileError,
            message: format!("Too many errors ({}), stopping compilation", self.max_errors),
            location: Location::new(0, 1, 1, 1),
            suggestions: Vec::new(),
            related: Vec::new(),
        };
        self.diagnostics.push(diagnostic);
    }
    
    /// Formats and prints a single diagnostic with rich formatting
    ///
    /// This private method handles the detailed formatting of individual diagnostics,
    /// including colored output, source code context, line markers, and suggestions.
    ///
    /// ### Arguments
    /// * `diagnostic` - The diagnostic to format and display
    /// * `line_info` - Line information for displaying source context
    fn report_diagnostic(&self, diagnostic: &Diagnostic, line_info: &LineInfo) {
        let severity_str = match diagnostic.severity {
            ErrorSeverity::Error => "error".red().bold(),
            ErrorSeverity::Warning => "warning".yellow().bold(),
            ErrorSeverity::Note => "note".blue().bold(),
        };
        
        let line = diagnostic.location.line;
        let col = diagnostic.location.column;
        let current_line_text = line_info.get_line_text(line).unwrap_or("<line not available>");
        
        eprintln!(
            "{} {}: {}",
            severity_str,
            diagnostic.error_code.to_string().bold(),
            diagnostic.message
        );
        
        eprintln!("  {} {}:{}:{}", "-->".yellow(), "main", line, col);
        
        let line_num_str = format!("{}", line);
        let indent_width = line_num_str.len() + 1;
        let indent = " ".repeat(indent_width);
        let pipe = "|".yellow();
        
        eprintln!("{indent}{}", pipe);
        eprintln!("{} {} {}", line_num_str.yellow(), pipe, current_line_text);
        
        let error_marker = " ".repeat(col.saturating_sub(1)) + &"^".repeat(diagnostic.location.length.max(1)).bold().red().to_string();
        eprintln!("{indent}{} {}", pipe, error_marker);
        
        for suggestion in &diagnostic.suggestions {
            eprintln!("{indent}{} {}: {}", pipe, "help".green().bold(), suggestion.message);
        }
        
        eprintln!();
    }
    
    /// Prints a summary of all collected diagnostics
    ///
    /// This private method displays a final summary showing the total count
    /// of errors and warnings encountered during compilation.
    fn report_summary(&self) {
        let mut parts = Vec::new();
        
        if self.error_count > 0 {
            parts.push(format!(
                "{} {}",
                self.error_count,
                if self.error_count == 1 { "error" } else { "errors" }
            ).red().to_string());
        }
        
        if self.warning_count > 0 {
            parts.push(format!(
                "{} {}",
                self.warning_count,
                if self.warning_count == 1 { "warning" } else { "warnings" }
            ).yellow().to_string());
        }
        
        if !parts.is_empty() {
            eprintln!("Compilation finished with {}", parts.join(", "));
        }
    }
    
    /// Removes and returns all collected diagnostics, resetting counters
    ///
    /// This method provides a way to extract all diagnostics while clearing
    /// the internal state. Useful for batch processing or transferring
    /// diagnostics to another system.
    ///
    /// ### Returns
    /// A vector containing all previously collected diagnostics
    pub fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        let diagnostics = std::mem::take(&mut self.diagnostics);
        self.error_count = 0;
        self.warning_count = 0;
        diagnostics
    }
}
