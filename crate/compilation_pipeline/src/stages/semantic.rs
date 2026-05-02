use crate::error::StageError;
use crate::stage::{CompilationStage, StageContext};
use slang_frontend::semantic_analysis;
use slang_ir::ast::Statement;
use slang_shared::DiagnosticEngine;

/// Semantic analysis stage that validates the AST
pub struct SemanticAnalysisStage;

impl CompilationStage for SemanticAnalysisStage {
    type Input = Vec<Statement>;
    type Output = Vec<Statement>;

    fn execute(
        &self,
        input: Self::Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, StageError> {
        // Notify observers about stage start
        context.observer_registry.notify_semantic_start(&input);

        // Use the shared compilation context from StageContext
        match semantic_analysis::execute(&input, &mut context.compilation_context) {
            Ok(()) => {
                context.observer_registry.notify_semantic_success(&input);
                Ok(input)
            }
            Err(errors) => {
                for error in &errors {
                    context.observer_registry.notify_semantic_error(error);
                    diagnostics.emit_compiler_error(error.clone());
                }
                Err(StageError::ExecutionFailed)
            }
        }
    }

    fn name(&self) -> &'static str {
        "Semantic Analysis"
    }

    fn is_critical(&self) -> bool {
        true
    }
}
