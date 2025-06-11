use std::collections::HashSet;
use slang_error::{CompilerError, ErrorCode};
use slang_shared::CompilationContext;
use super::error::SemanticAnalysisError;

/// A centralized error collector that handles error creation, formatting, and deduplication
/// 
/// This provides a single source of truth for error handling in the semantic analysis system,
/// ensuring consistent error formatting and efficient deduplication.
pub struct ErrorCollector {
    errors: Vec<CompilerError>,
    seen_errors: HashSet<ErrorKey>,
}

/// Key for error deduplication that groups semantically similar errors
#[derive(Hash, PartialEq, Eq, Clone)]
struct ErrorKey {
    code: ErrorCode,
    line: usize,
    column: usize,
    message: String,
}

impl ErrorCollector {
    /// Create a new error collector
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            seen_errors: HashSet::new(),
        }
    }

    /// Add a semantic analysis error to the collection
    /// 
    /// # Arguments
    /// * `error` - The semantic analysis error to add
    /// * `context` - The compilation context for error conversion
    /// 
    /// # Returns
    /// `true` if the error was added (not a duplicate), `false` if it was deduplicated
    pub fn add_semantic_error(&mut self, error: SemanticAnalysisError, context: &CompilationContext) -> bool {
        let compiler_error = error.to_compiler_error(context);
        self.add_compiler_error(compiler_error)
    }

    /// Add a compiler error directly to the collection
    /// 
    /// # Arguments
    /// * `error` - The compiler error to add
    /// 
    /// # Returns
    /// `true` if the error was added (not a duplicate), `false` if it was deduplicated
    pub fn add_compiler_error(&mut self, error: CompilerError) -> bool {
        let key = ErrorKey {
            code: error.error_code,
            line: error.line,
            column: error.column,
            message: error.message.clone(),
        };

        if self.seen_errors.insert(key) {
            self.errors.push(error);
            true
        } else {
            false
        }
    }

    /// Get all collected errors
    pub fn into_errors(self) -> Vec<CompilerError> {
        self.errors
    }

    /// Check if any errors have been collected
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get the number of unique errors collected
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

}

impl Default for ErrorCollector {
    fn default() -> Self {
        Self::new()
    }
}
