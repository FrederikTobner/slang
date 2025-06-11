
use crate::compilation_pipeline::{CompilationResult, compile_to_bytecode};

/// Configuration options for compilation
#[derive(Default)]
pub struct CompileOptions {
    /// Enable error recovery mode
    pub recovery_mode: bool,
    /// File name for better error reporting
    pub file_name: Option<String>,
}

/// High-level compiler facade that encapsulates the compilation pipeline
/// 
/// This provides a cleaner API for compilation that can be reused in different contexts
/// such as CLI, language servers, or testing environments.
pub struct Compiler;

impl Compiler {
    /// Create a new compiler instance
    pub fn new() -> Self {
        Self
    }
    
    /// Compile source code to bytecode
    ///
    /// ### Arguments
    /// * `source` - The source code to compile
    /// * `options` - Compilation options
    ///
    /// ### Returns
    /// The compilation result with diagnostics
    pub fn compile_source<'a>(&self, source: &'a str, options: CompileOptions) -> CompilationResult<'a> {
        compile_to_bytecode(source, options.file_name, options.recovery_mode)
    }
}