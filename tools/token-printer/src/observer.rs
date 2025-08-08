use crate::formatters::TokenFormatter;
use slang_compilation_pipeline::SlangSourceFile;
use slang_compilation_pipeline::observer::StageObserver;
use slang_frontend::Token;

/// Type-safe token printer observer implementing StageObserver for the tokenization stage
pub struct TokenPrinter {
    formatter: Box<dyn TokenFormatter>,
    file_name: String,
}

impl TokenPrinter {
    pub fn new(formatter: Box<dyn TokenFormatter>, file_name: String) -> Self {
        Self {
            formatter,
            file_name,
        }
    }
}

impl StageObserver<SlangSourceFile, Vec<Token>> for TokenPrinter {
    fn on_stage_success(&self, tokens: &Vec<Token>) {
        self.formatter.format_tokens(tokens, &self.file_name);
    }
}
