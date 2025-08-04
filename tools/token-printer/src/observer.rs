use crate::formatters::TokenFormatter;
use slang_compilation_pipeline::pipeline::observers::StageObserver;
use slang_frontend::Token;

/// Type-safe token printer observer using the new generic observer system
pub struct TokenPrinterObserver {
    formatter: Box<dyn TokenFormatter>,
    file_name: String,
}

impl TokenPrinterObserver {
    pub fn new(formatter: Box<dyn TokenFormatter>, file_name: String) -> Self {
        Self { formatter, file_name }
    }
}

impl StageObserver<String, Vec<Token>> for TokenPrinterObserver {
    fn on_stage_success(&self, tokens: &Vec<Token>) {
        self.formatter.format_tokens(tokens, &self.file_name);
    }
}
