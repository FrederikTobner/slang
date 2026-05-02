use crate::error::StageError;
use crate::stage::{CompilationStage, StageContext};
use slang_backend::{bytecode::Chunk, codegen};
use slang_ir::ast::Statement;
use slang_shared::DiagnosticEngine;

/// Code generation stage that produces bytecode
pub struct CodeGenerationStage;

impl CompilationStage for CodeGenerationStage {
    type Input = Vec<Statement>;
    type Output = Chunk;

    fn execute(
        &self,
        input: Self::Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, StageError> {
        // Notify observers about stage start
        context.observer_registry.notify_codegen_start(&input);

        // Use the real code generator
        match codegen::generate_bytecode(&input) {
            Ok(chunk) => {
                // Notify observers about successful completion
                context.observer_registry.notify_codegen_success(&chunk);
                Ok(chunk)
            }
            Err(errors) => {
                // Emit all code generation errors to the diagnostic engine
                for error in &errors {
                    context.observer_registry.notify_codegen_error(error);
                    diagnostics.emit_compiler_error(error.clone());
                }
                Err(StageError::ExecutionFailed)
            }
        }
    }

    fn name(&self) -> &'static str {
        "Code Generation"
    }

    fn is_critical(&self) -> bool {
        true
    }
}
