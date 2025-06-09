use slang_error::{ErrorCode, ErrorSeverity, LineInfo, ErrorContext};
use slang_ir::location::Location;
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: ErrorSeverity,
    pub error_code: ErrorCode,
    pub message: String,
    pub location: Location,
    pub suggestions: Vec<Suggestion>,
    pub related: Vec<Diagnostic>,
}

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub message: String,
    pub replacement: Option<String>,
    pub location: Option<Location>,
}

pub struct DiagnosticEngine<'a> {
    diagnostics: Vec<Diagnostic>,
    error_count: usize,
    warning_count: usize,
    max_errors: usize,
    recovery_mode: bool,
    context: ErrorContext<'a>,
}

impl<'a> DiagnosticEngine<'a> {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            error_count: 0,
            warning_count: 0,
            max_errors: 100,
            recovery_mode: false,
            context: ErrorContext::default(),
        }
    }
    
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
    
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }
    
    pub fn error_count(&self) -> usize {
        self.error_count
    }
    
    pub fn warning_count(&self) -> usize {
        self.warning_count
    }
    
    pub fn finish(self) -> Result<(), Vec<Diagnostic>> {
        if self.has_errors() {
            Err(self.diagnostics)
        } else {
            Ok(())
        }
    }
    
    pub fn set_recovery_mode(&mut self, enabled: bool) {
        self.recovery_mode = enabled;
    }
    
    pub fn is_recovery_mode(&self) -> bool {
        self.recovery_mode
    }
    
    pub fn set_file_name(&mut self, file_name: String) {
        self.context.file_name = Some(file_name);
    }
    
    pub fn set_source_text(&mut self, source_text: &'a str) {
        self.context.source_text = Some(source_text);
    }
    
    pub fn set_max_errors(&mut self, max_errors: usize) {
        self.max_errors = max_errors;
    }
    
    pub fn into_errors(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
    
    pub fn report_all(&self, source: &str) {
        let line_info = LineInfo::new(source);
        for diagnostic in &self.diagnostics {
            self.report_diagnostic(diagnostic, &line_info);
        }
        
        if self.error_count > 0 || self.warning_count > 0 {
            self.report_summary();
        }
    }
    
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
    
    pub fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        let diagnostics = std::mem::take(&mut self.diagnostics);
        self.error_count = 0;
        self.warning_count = 0;
        diagnostics
    }
}
