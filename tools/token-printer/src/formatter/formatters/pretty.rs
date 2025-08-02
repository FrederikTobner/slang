use colored::Colorize;
use slang_frontend::Token;
use super::TokenFormatter;

/// Pretty formatter with colored output
pub struct PrettyFormatter;

impl TokenFormatter for PrettyFormatter {
    fn format_tokens(&self, tokens: &[Token], file_name: &str) {
        println!("{}", format!("=== Tokens for {} ===", file_name).bright_cyan().bold());
        
        for (i, token) in tokens.iter().enumerate() {
            let token_type_str = format!("{:15}", format!("{:?}", token.token_type));
            let lexeme_str = if token.lexeme.is_empty() {
                "<empty>".dimmed().to_string()
            } else {
                format!("'{}'", token.lexeme)
            };
            let position_str = format!("pos: {}", token.pos).dimmed();

            println!(
                "{:3}: {} {} ({})",
                i.to_string().bright_black(),
                token_type_str.cyan(),
                lexeme_str,
                position_str
            );
        }
        
        println!("{}", format!("=== {} tokens total ===", tokens.len()).bright_cyan().bold());
    }
}
