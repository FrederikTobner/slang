use std::error::Error;
use std::fmt;

/// Strategy for handling compilation errors
#[derive(Debug, Clone)]
pub enum ErrorStrategy {
    /// Stop compilation on first error
    FailFast,
    
    /// Continue compilation, collecting errors
    Recover { 
        /// Whether to continue on non-critical stage failures
        continue_on_non_critical: bool 
    },
}

impl Default for ErrorStrategy {
    fn default() -> Self {
        Self::FailFast
    }
}

/// Result of error recovery decision
#[derive(Debug)]
pub enum RecoveryAction {
    /// Stop compilation immediately
    Stop,
    /// Continue with empty/default output
    ContinueWithDefault,
    /// Continue with the provided output  
    ContinueWith(Box<dyn std::any::Any>),
}

/// Pipeline-specific error type
#[derive(Debug)]
pub enum PipelineError {
    /// Stage execution failed
    StageError {
        stage_name: String,
        error: Box<dyn Error + Send + Sync>,
    },
    /// Type mismatch between stages
    TypeMismatch {
        stage_name: String,
        expected: String,
        found: String,
    },
    /// Invalid pipeline configuration
    InvalidConfiguration(String),
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineError::StageError { stage_name, error } => {
                write!(f, "Stage '{}' failed: {}", stage_name, error)
            }
            PipelineError::TypeMismatch { stage_name, expected, found } => {
                write!(f, "Type mismatch in stage '{}': expected {}, found {}", stage_name, expected, found)
            }
            PipelineError::InvalidConfiguration(msg) => {
                write!(f, "Invalid pipeline configuration: {}", msg)
            }
        }
    }
}

impl Error for PipelineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PipelineError::StageError { error, .. } => Some(error.as_ref()),
            _ => None,
        }
    }
}
