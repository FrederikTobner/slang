use std::any::Any;
use slang_shared::{DiagnosticEngine, CompilationContext};
use slang_frontend::{CompilerError, ErrorCode};

/// Core trait for compilation stages
/// 
/// Each stage transforms input data to output data, potentially modifying
/// the compilation context and emitting diagnostics. Stages should be stateless 
/// and pure transformations, with errors reported through the DiagnosticEngine.
pub trait CompilationStage: Send + Sync {
    /// The input type for this stage
    type Input: 'static;
    /// The output type for this stage  
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
    pub observer_registry: crate::pipeline::observers::ObserverRegistry,
}

impl StageContext {
    pub fn new(source: String, file_name: Option<String>) -> Self {
        Self { 
            source, 
            file_name,
            compilation_context: CompilationContext::new(),
            observer_registry: crate::pipeline::observers::ObserverRegistry::new(),
        }
    }
    
    pub fn with_observer_registry(source: String, file_name: Option<String>, observer_registry: crate::pipeline::observers::ObserverRegistry) -> Self {
        Self { 
            source, 
            file_name,
            compilation_context: CompilationContext::new(),
            observer_registry,
        }
    }
}

/// Type-erased wrapper for compilation stages
/// Enables dynamic dispatch while preserving type safety
pub struct AnyStage {
    stage: Box<dyn AnyStageImpl>,
    name: &'static str,
    is_critical: bool,
}

impl AnyStage {
    pub fn new<S: CompilationStage + 'static>(stage: S) -> Self {
        let name = stage.name();
        let is_critical = stage.is_critical();
        Self {
            stage: Box::new(TypedStageWrapper { stage }),
            name,
            is_critical,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn is_critical(&self) -> bool {
        self.is_critical
    }

    pub fn execute(&self, input: Box<dyn Any>, context: &mut StageContext, diagnostics: &mut DiagnosticEngine) -> Result<Box<dyn Any>, ()> {
        self.stage.execute_any(input, context, diagnostics)
    }
}

/// Internal trait for type-erased stage execution
trait AnyStageImpl: Send + Sync {
    fn execute_any(&self, input: Box<dyn Any>, context: &mut StageContext, diagnostics: &mut DiagnosticEngine) -> Result<Box<dyn Any>, ()>;
}

/// Wrapper that implements AnyStageImpl for any CompilationStage
struct TypedStageWrapper<S: CompilationStage> {
    stage: S,
}

impl<S: CompilationStage + 'static> AnyStageImpl for TypedStageWrapper<S> {
    fn execute_any(&self, input: Box<dyn Any>, context: &mut StageContext, diagnostics: &mut DiagnosticEngine) -> Result<Box<dyn Any>, ()> {
        // Downcast input to the expected type
        let typed_input = input
            .downcast::<S::Input>()
            .map_err(|_| {
                // Emit type mismatch error to diagnostics
                let error = CompilerError::new(
                    ErrorCode::UnexpectedToken,
                    format!("Type mismatch in stage '{}': expected input type", self.stage.name()),
                    1, // line
                    1, // column
                    0, // position
                    None, // token_length
                );
                diagnostics.emit_compiler_error(error);
            })?;

        // Execute the stage
        let output = self.stage.execute(*typed_input, context, diagnostics)?;

        // Box the output as Any
        Ok(Box::new(output) as Box<dyn Any>)
    }
}
