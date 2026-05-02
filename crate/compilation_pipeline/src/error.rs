/// Strategy for handling compilation errors
#[derive(Debug, Clone)]
pub enum ErrorStrategy {
    /// Stop compilation on first error
    FailFast,

    /// Continue compilation, collecting errors
    Recover {
        /// Whether to continue on non-critical stage failures
        continue_on_non_critical: bool,
    },
}

impl Default for ErrorStrategy {
    fn default() -> Self {
        Self::FailFast
    }
}

/// Error type for stage execution failures
#[derive(Debug, Clone)]
pub enum StageError {
    /// The stage failed to execute (errors emitted to diagnostics)
    ExecutionFailed,
    /// Critical error that should halt the pipeline
    Critical(String),
    /// Internal error in the stage implementation
    Internal(String),
}

impl std::fmt::Display for StageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StageError::ExecutionFailed => write!(f, "Stage execution failed"),
            StageError::Critical(msg) => write!(f, "Critical stage error: {msg}"),
            StageError::Internal(msg) => write!(f, "Internal stage error: {msg}"),
        }
    }
}

impl std::error::Error for StageError {}
