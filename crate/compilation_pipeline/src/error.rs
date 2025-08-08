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
