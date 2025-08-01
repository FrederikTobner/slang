use std::any::Any;
use slang_shared::DiagnosticEngine;

/// Result of pipeline execution
pub enum CompilationResult<'a> {
    /// Compilation succeeded
    Success {
        /// The final output data
        output: Box<dyn Any>,
        /// Diagnostic engine with any warnings/info
        diagnostics: DiagnosticEngine<'a>,
    },
    /// Compilation failed
    Failed {
        /// Diagnostic engine with errors
        diagnostics: DiagnosticEngine<'a>,
    },
}

impl<'a> CompilationResult<'a> {
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
    pub fn output(self) -> Option<Box<dyn Any>> {
        match self {
            CompilationResult::Success { output, .. } => Some(output),
            CompilationResult::Failed { .. } => None,
        }
    }

    /// Extract the output data if compilation succeeded, downcasting to the expected type
    /// 
    /// Returns None if compilation failed or the type doesn't match
    pub fn output_as<T: 'static>(self) -> Option<T> {
        self.output()?.downcast::<T>().ok().map(|boxed| *boxed)
    }

    /// Extract the diagnostics, consuming the result
    pub fn into_diagnostics(self) -> DiagnosticEngine<'a> {
        match self {
            CompilationResult::Success { diagnostics, .. } => diagnostics,
            CompilationResult::Failed { diagnostics } => diagnostics,
        }
    }
}

/// Intermediate result that represents a stage in a compilation pipeline
/// 
/// This is used internally during pipeline execution to chain stages together.
pub enum PipelineStage<'a, T> {
    /// Stage completed successfully
    Success {
        /// The pipeline state for the next stage
        pipeline: crate::compilation_pipeline::CompilationPipeline<'a>,
        /// The data produced by this stage
        data: T,
    },
    /// Stage failed
    Failed {
        /// The pipeline state with error information
        pipeline: crate::compilation_pipeline::CompilationPipeline<'a>,
    },
}

impl<'a, T: 'static> PipelineStage<'a, T> {
    /// Chain another stage if this one succeeded
    pub fn and_then<U, F>(self, f: F) -> PipelineStage<'a, U>
    where
        F: FnOnce(crate::compilation_pipeline::CompilationPipeline<'a>, T) -> PipelineStage<'a, U>,
    {
        match self {
            PipelineStage::Success { pipeline, data } => f(pipeline, data),
            PipelineStage::Failed { pipeline } => PipelineStage::Failed { pipeline },
        }
    }

    /// Extract the pipeline, consuming the result
    pub fn into_pipeline(self) -> crate::compilation_pipeline::CompilationPipeline<'a> {
        match self {
            PipelineStage::Success { pipeline, .. } => pipeline,
            PipelineStage::Failed { pipeline } => pipeline,
        }
    }

    /// Check if the stage succeeded
    pub fn is_success(&self) -> bool {
        matches!(self, PipelineStage::Success { .. })
    }

    /// Check if the stage failed
    pub fn is_failed(&self) -> bool {
        matches!(self, PipelineStage::Failed { .. })
    }

    /// Convert to a CompilationResult, consuming the stage
    pub fn into_result(self) -> CompilationResult<'a> {
        match self {
            PipelineStage::Success { pipeline, data } => CompilationResult::Success {
                output: Box::new(data) as Box<dyn Any>,
                diagnostics: pipeline.into_diagnostics(),
            },
            PipelineStage::Failed { pipeline } => CompilationResult::Failed {
                diagnostics: pipeline.into_diagnostics(),
            },
        }
    }
}
