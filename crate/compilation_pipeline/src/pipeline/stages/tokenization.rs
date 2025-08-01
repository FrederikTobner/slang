use crate::pipeline::stage::{CompilationStage, StageContext};
use slang_frontend::{lexer::Lexer, token::Token};
use slang_shared::DiagnosticEngine;

/// Tokenization stage that converts source code to tokens
pub struct TokenizationStage;

impl CompilationStage for TokenizationStage {
    type Input = String;
    type Output = Vec<Token>;

    fn execute(&self, input: Self::Input, context: &mut StageContext, diagnostics: &mut DiagnosticEngine) -> Result<Self::Output, ()> {
        // Notify observers about stage start
        context.observer_registry.notify_tokenization_start(&input);
        
        let lexer = Lexer::new(&input);
        
        match lexer.tokenize() {
            Ok(result) => {
                // Notify observers about successful completion
                context.observer_registry.notify_tokenization_success(&result.tokens);
                Ok(result.tokens)
            },
            Err(errors) => {
                for error in errors {
                    // Notify observers about errors
                    context.observer_registry.notify_tokenization_error(&error);
                    diagnostics.emit_compiler_error(error);
                }
                Err(())
            }
        }
    }

    fn name(&self) -> &'static str {
        "Tokenization"
    }
}
