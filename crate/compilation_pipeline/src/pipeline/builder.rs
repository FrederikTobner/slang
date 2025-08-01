use crate::pipeline::{
    stage::{AnyStage, StageContext},
    error::{ErrorStrategy, PipelineError},
    result::{CompilationResult},
    stages::*,
    observers::*,
};
use slang_shared::DiagnosticEngine;
use std::any::Any;

/// Builder for constructing compilation pipelines
pub struct PipelineBuilder<'a> {
    stages: Vec<AnyStage>,
    observer_registry: crate::pipeline::observers::ObserverRegistry,
    error_strategy: ErrorStrategy,
    source: &'a str,
    file_name: Option<String>,
}

impl<'a> PipelineBuilder<'a> {
    /// Create a new pipeline builder
    pub fn new(source: &'a str) -> Self {
        Self {
            stages: Vec::new(),
            observer_registry: crate::pipeline::observers::ObserverRegistry::new(),
            error_strategy: ErrorStrategy::default(),
            source,
            file_name: None,
        }
    }

    /// Add a compilation stage to the pipeline
    pub fn add_stage<S: crate::pipeline::stage::CompilationStage + 'static>(mut self, stage: S) -> Self {
        self.stages.push(AnyStage::new(stage));
        self
    }
    
    /// Add a tokenization observer (new type-safe system)
    pub fn add_tokenization_observer<T>(mut self, observer: T) -> Self 
    where 
        T: crate::pipeline::observers::StageObserver<String, Vec<slang_frontend::Token>> + 'static
    {
        self.observer_registry.add_tokenization_observer(observer);
        self
    }
    
    /// Add a parsing observer (new type-safe system)
    pub fn add_parsing_observer<T>(mut self, observer: T) -> Self 
    where 
        T: crate::pipeline::observers::StageObserver<Vec<slang_frontend::Token>, Vec<slang_ir::ast::Statement>> + 'static
    {
        self.observer_registry.add_parsing_observer(observer);
        self
    }
    
    /// Add a semantic analysis observer (new type-safe system)
    pub fn add_semantic_observer<T>(mut self, observer: T) -> Self 
    where 
        T: crate::pipeline::observers::StageObserver<Vec<slang_ir::ast::Statement>, Vec<slang_ir::ast::Statement>> + 'static
    {
        self.observer_registry.add_semantic_observer(observer);
        self
    }
    
    /// Add a code generation observer (new type-safe system)
    pub fn add_codegen_observer<T>(mut self, observer: T) -> Self 
    where 
        T: crate::pipeline::observers::StageObserver<Vec<slang_ir::ast::Statement>, slang_backend::bytecode::Chunk> + 'static
    {
        self.observer_registry.add_codegen_observer(observer);
        self
    }

    /// Set the error handling strategy
    pub fn with_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.error_strategy = strategy;
        self
    }

    /// Set the file name for error reporting
    pub fn with_file_name(mut self, file_name: String) -> Self {
        self.file_name = Some(file_name);
        self
    }

    /// Build the final pipeline
    pub fn build(self) -> Pipeline<'a> {
        Pipeline {
            stages: self.stages,
            observer_registry: self.observer_registry,
            error_strategy: self.error_strategy,
            source: self.source,
            file_name: self.file_name,
        }
    }

    /// Convenience method: create a standard compilation pipeline
    pub fn standard(source: &'a str) -> Self {
        Self::new(source)
            .add_stage(TokenizationStage)
            .add_stage(ParsingStage)
            .add_stage(SemanticAnalysisStage)
            .add_stage(CodeGenerationStage)
    }

    /// Convenience method: create a pipeline with debug features
    pub fn with_debug(self) -> Self {
        // Add debug observers based on feature flags
        #[cfg(feature = "print-ast")]
        let builder = self.add_parsing_observer(ASTPrintObserver::new())
                         .add_semantic_observer(ASTPrintObserver::new());
        #[cfg(not(feature = "print-ast"))]  
        let builder = self;
        
        #[cfg(feature = "print-byte_code")]
        let builder = builder.add_codegen_observer(BytecodePrintObserver::new());
        
        builder
    }

    /// Add debug observers regardless of feature flags (for tools)
    pub fn with_debug_forced(self) -> Self {
        self.add_parsing_observer(ASTPrintObserver::new())
            .add_semantic_observer(ASTPrintObserver::new())
            .add_codegen_observer(BytecodePrintObserver::new())
    }
}

/// Compiled pipeline ready for execution
pub struct Pipeline<'a> {
    stages: Vec<AnyStage>,
    observer_registry: crate::pipeline::observers::ObserverRegistry,
    error_strategy: ErrorStrategy,
    source: &'a str,
    file_name: Option<String>,
}

impl<'a> Pipeline<'a> {
    /// Execute the complete pipeline
    pub fn execute(self) -> CompilationResult<'a> {
        let mut diagnostics = DiagnosticEngine::new();
        
        // Set up diagnostics
        if let Some(ref name) = self.file_name {
            diagnostics.set_file_name(name.clone());
        }
        diagnostics.set_source_text(self.source);
        
        // Set recovery mode based on error strategy
        match self.error_strategy {
            ErrorStrategy::FailFast => diagnostics.set_recovery_mode(false),
            ErrorStrategy::Recover { .. } => diagnostics.set_recovery_mode(true),
        }

        let mut stage_context = StageContext::with_observer_registry(
            self.source.to_string(),
            self.file_name.clone(),
            self.observer_registry,
        );

        // Start with source code as initial input
        let mut current_output: Box<dyn Any> = Box::new(self.source.to_string());
        let mut pipeline_failed = false;

        // Execute each stage
        for stage in &self.stages {
            // Execute the stage
            let stage_result = stage.execute(current_output, &mut stage_context, &mut diagnostics);
            match stage_result {
                Ok(output) => {
                    current_output = output;
                }
                Err(()) => {
                    // Stage failed - errors already emitted to diagnostics
                    pipeline_failed = true;
                    
                    match self.error_strategy {
                        ErrorStrategy::FailFast => {
                            // Create a default output for failed cases
                            current_output = Box::new(String::new());
                            break; // Exit the loop early
                        }
                        ErrorStrategy::Recover { continue_on_non_critical } => {
                            if stage.is_critical() || !continue_on_non_critical {
                                // Create a default output for failed cases
                                current_output = Box::new(String::new());
                                break; // Exit the loop early
                            }
                            // For non-critical stages, continue with empty output
                            // This is a simplification - in practice we'd want stage-specific defaults
                            current_output = Box::new(Vec::<()>::new());
                        }
                    }
                }
            }
        }

        // Drop stage_context to release mutable borrows
        drop(stage_context);
        
        // Check if we have errors before moving diagnostics
        let has_errors = pipeline_failed || diagnostics.error_count() > 0;
        
        // Create final result based on whether pipeline failed
        let result = if has_errors {
            CompilationResult::Failed { diagnostics }
        } else {
            CompilationResult::Success {
                output: current_output,
                diagnostics,
            }
        };

        result
    }

    /// Execute pipeline up to a specific stage  
    pub fn execute_until<T: 'static>(self, stage_name: &str) -> Result<T, PipelineError> {
        let mut diagnostics = DiagnosticEngine::new();
        
        let mut stage_context = StageContext::new(
            self.source.to_string(),
            self.file_name.clone(),
        );

        let mut current_output: Box<dyn Any> = Box::new(self.source.to_string());

        for stage in &self.stages {
            // Execute the stage
            current_output = stage.execute(current_output, &mut stage_context, &mut diagnostics)
                .map_err(|_| PipelineError::StageError {
                    stage_name: stage.name().to_string(),
                    error: Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Stage failed")),
                })?;

            // Check if this is the target stage
            if stage.name() == stage_name {
                return current_output.downcast::<T>()
                    .map(|boxed| *boxed)
                    .map_err(|_| PipelineError::TypeMismatch {
                        stage_name: stage_name.to_string(),
                        expected: std::any::type_name::<T>().to_string(),
                        found: "unknown".to_string(),
                    });
            }
        }

        Err(PipelineError::InvalidConfiguration(
            format!("Stage '{}' not found in pipeline", stage_name)
        ))
    }

    /// Execute with intermediate results (for debugging)
    pub fn execute_with_intermediates(self) -> (CompilationResult<'a>, Vec<Box<dyn Any>>) {
        let intermediates = Vec::new();
        
        // For now, just execute normally and return empty intermediates
        // This is a placeholder for more sophisticated intermediate result collection
        let result = self.execute();
        
        (result, intermediates)
    }
}
