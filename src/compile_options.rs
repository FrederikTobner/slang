/// Configuration options for compilation
#[derive(Default)]
pub struct CompileOptions {
    /// Enable error recovery mode
    pub recovery_mode: bool,
    /// File name for better error reporting
    pub file_name: Option<String>,
}
