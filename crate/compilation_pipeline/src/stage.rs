use slang_shared::{DiagnosticEngine, CompilationContext};

/// Core trait for compilation stages using associated types
/// 
/// Each stage transforms input data to output data, potentially modifying
/// the compilation context and emitting diagnostics. Stages should be stateless 
/// and pure transformations, with errors reported through the DiagnosticEngine.
///
/// This design uses associated types to define the input and output types for each stage,
/// providing clear, simple type signatures while maintaining flexibility.
pub trait CompilationStage: Send + Sync + 'static {
    /// The input type this stage accepts
    type Input: 'static;
    
    /// The output type this stage produces
    type Output: 'static;

    /// Execute the compilation stage
    /// 
    /// Takes a separate DiagnosticEngine to emit errors directly.
    /// Returns Ok(output) on success, or Err(()) on failure (with errors emitted to diagnostics).
    fn execute(&self, input: Self::Input, context: &mut StageContext, diagnostics: &mut DiagnosticEngine) -> Result<Self::Output, ()>;
    
    /// Get the human-readable name of this stage
    fn name(&self) -> &'static str;
    
    /// Whether failure of this stage should stop compilation entirely
    /// Non-critical stages can continue compilation in recovery mode
    fn is_critical(&self) -> bool { true }
}

/// Context passed to each pipeline stage with owned values to avoid lifetime issues
pub struct StageContext {
    /// The source code being compiled
    pub source: String,
    /// Optional file name for the source being compiled
    pub file_name: Option<String>,
    /// Compilation context shared across all stages
    pub compilation_context: CompilationContext,
    /// Observer registry for type-safe pipeline monitoring
    pub observer_registry: crate::observer::ObserverRegistry,
}

impl StageContext {
    pub fn new(source: String, file_name: Option<String>) -> Self {
        Self { 
            source, 
            file_name,
            compilation_context: CompilationContext::new(),
            observer_registry: crate::observer::ObserverRegistry::new(),
        }
    }
    
    pub fn with_observer_registry(source: String, file_name: Option<String>, observer_registry: crate::observer::ObserverRegistry) -> Self {
        Self { 
            source, 
            file_name,
            compilation_context: CompilationContext::new(),
            observer_registry,
        }
    }
}
