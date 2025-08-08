use slang_shared::DiagnosticEngine;

/// Result of pipeline execution with typed output
pub enum CompilationResult<'a, T> {
    /// Compilation succeeded
    Success {
        /// The final output data
        output: T,
        /// Diagnostic engine with any warnings/info
        diagnostics: DiagnosticEngine<'a>,
    },
    /// Compilation failed
    Failed {
        /// Diagnostic engine with errors
        diagnostics: DiagnosticEngine<'a>,
    },
}

impl<'a, T> CompilationResult<'a, T> {
    /// Check if the compilation succeeded
    pub fn is_success(&self) -> bool {
        matches!(self, CompilationResult::Success { .. })
    }

    /// Check if the compilation failed
    pub fn is_failed(&self) -> bool {
        matches!(self, CompilationResult::Failed { .. })
    }

    /// Get the diagnostics engine regardless of success/failure
    pub fn diagnostics(&self) -> &DiagnosticEngine<'a> {
        match self {
            CompilationResult::Success { diagnostics, .. } => diagnostics,
            CompilationResult::Failed { diagnostics } => diagnostics,
        }
    }

    /// Extract the output data if compilation succeeded
    ///
    /// Returns None if compilation failed
    pub fn output(self) -> Option<T> {
        match self {
            CompilationResult::Success { output, .. } => Some(output),
            CompilationResult::Failed { .. } => None,
        }
    }

    /// Extract the diagnostics, consuming the result
    pub fn into_diagnostics(self) -> DiagnosticEngine<'a> {
        match self {
            CompilationResult::Success { diagnostics, .. } => diagnostics,
            CompilationResult::Failed { diagnostics } => diagnostics,
        }
    }
}
