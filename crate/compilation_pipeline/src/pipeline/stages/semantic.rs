use crate::pipeline::stage::{CompilationStage, StageContext};
use slang_ir::ast::Statement;
use slang_shared::DiagnosticEngine;
use slang_frontend::semantic_analysis;

/// Semantic analysis stage that validates the AST
pub struct SemanticAnalysisStage;

impl CompilationStage for SemanticAnalysisStage {
    type Input = Vec<Statement>;
    type Output = Vec<Statement>;

    fn execute(&self, input: Self::Input, context: &mut StageContext, diagnostics: &mut DiagnosticEngine) -> Result<Self::Output, ()> {
        // Notify observers about stage start
        context.observer_registry.notify_semantic_start(&input);
        
        // Use the shared compilation context from StageContext
        match semantic_analysis::execute(&input, &mut context.compilation_context) {
            Ok(()) => {
                // Notify observers about successful completion
                context.observer_registry.notify_semantic_success(&input);
                // Semantic analysis passed, return the statements
                Ok(input)
            }
            Err(errors) => {
                // Emit all semantic errors to the diagnostic engine
                for error in &errors {
                    context.observer_registry.notify_semantic_error(error);
                    diagnostics.emit_compiler_error(error.clone());
                }
                Err(())
            }
        }
    }

    fn name(&self) -> &'static str {
        "Semantic Analysis"
    }

    fn is_critical(&self) -> bool {
        false // Allow continuing to codegen for better error reporting
    }
}
