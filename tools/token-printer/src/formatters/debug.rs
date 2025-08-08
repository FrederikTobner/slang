use colored::Colorize;
use slang_frontend::Token;
use super::TokenFormatter;

/// Debug formatter with raw debug output
pub struct DebugFormatter;

impl TokenFormatter for DebugFormatter {
    fn format_tokens(&self, tokens: &[Token], file_name: &str) {
        println!("{}", format!("=== Debug tokens for {file_name} ===").bright_yellow().bold());
        
        for (i, token) in tokens.iter().enumerate() {
            println!("{:3}: {:?} '{}' (pos: {})", i, token.token_type, token.lexeme, token.pos);
        }
        
        println!("{}", format!("=== {} tokens total ===", tokens.len()).bright_yellow().bold());
    }
}
