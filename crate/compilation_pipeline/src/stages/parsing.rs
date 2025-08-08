use crate::error::StageError;
use crate::stage::{CompilationStage, StageContext};
use slang_frontend::{LineInfo, Token, parser::Parser};
use slang_ir::ast::Statement;
use slang_shared::DiagnosticEngine;

/// Parsing stage that converts tokens to AST
pub struct ParsingStage;

impl CompilationStage for ParsingStage {
    type Input = Vec<Token>;
    type Output = Vec<Statement>;

    fn execute(
        &self,
        input: Self::Input,
        context: &mut StageContext,
        diagnostics: &mut DiagnosticEngine,
    ) -> Result<Self::Output, StageError> {
        // Notify observers about stage start
        context.observer_registry.notify_parsing_start(&input);

        // Create line info from the source
        let line_info = LineInfo::new(&context.source);

        // Use the shared compilation context from StageContext
        let mut parser = Parser::new(&input, &line_info, &mut context.compilation_context);

        match parser.parse() {
            Ok(statements) => {
                // Notify observers about successful completion
                context
                    .observer_registry
                    .notify_parsing_success(&statements);
                Ok(statements)
            }
            Err(errors) => {
                // Emit all parsing errors to the diagnostic engine
                for error in &errors {
                    context.observer_registry.notify_parsing_error(error);
                    diagnostics.emit_compiler_error(error.clone());
                }
                Err(StageError::ExecutionFailed)
            }
        }
    }

    fn name(&self) -> &'static str {
        "Parsing"
    }

    fn is_critical(&self) -> bool {
        true
    }
}
