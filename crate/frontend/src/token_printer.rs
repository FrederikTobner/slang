use crate::token::{Token, Tokentype};
use colored::Colorize;

/// A utility for printing tokens in a human-readable format
pub struct TokenPrinter;

impl TokenPrinter {
    /// Creates a new token printer
    pub fn new() -> Self {
        TokenPrinter
    }

    /// Prints a list of tokens with formatting
    ///
    /// ### Arguments
    ///
    /// * `tokens` - The tokens to print
    pub fn print(&self, tokens: &[Token]) {
        println!("{}", "=== TOKENS ===".bright_cyan().bold());
        
        for (i, token) in tokens.iter().enumerate() {
            self.print_token(i, token);
        }
        
        println!("{}", "=== END TOKENS ===".bright_cyan().bold());
    }

    /// Prints a single token with formatting
    ///
    /// ### Arguments
    ///
    /// * `index` - The index of the token in the token list
    /// * `token` - The token to print
    fn print_token(&self, index: usize, token: &Token) {
        let token_type_str = self.format_token_type(&token.token_type);
        let lexeme_str = self.format_lexeme(&token.lexeme, &token.token_type);
        let position_str = format!("pos: {}", token.pos).dimmed();
        
        println!(
            "{:3}: {} {} ({})",
            index.to_string().bright_black(),
            token_type_str,
            lexeme_str,
            position_str
        );
    }

    /// Formats the token type with appropriate colors
    ///
    /// ### Arguments
    ///
    /// * `token_type` - The token type to format
    ///
    /// ### Returns
    ///
    /// A formatted string with colors
    fn format_token_type(&self, token_type: &Tokentype) -> String {
        match token_type {
            // Keywords
            Tokentype::Let | Tokentype::Mut | Tokentype::Fn | Tokentype::Return 
            | Tokentype::If | Tokentype::Else | Tokentype::Struct => {
                format!("{:15}", format!("{:?}", token_type)).blue().bold().to_string()
            }
            
            // Literals
            Tokentype::IntegerLiteral | Tokentype::FloatLiteral | Tokentype::StringLiteral 
            | Tokentype::BooleanLiteral => {
                format!("{:15}", format!("{:?}", token_type)).green().to_string()
            }
            
            // Identifiers
            Tokentype::Identifier => {
                format!("{:15}", format!("{:?}", token_type)).cyan().to_string()
            }
            
            // Operators
            Tokentype::Plus | Tokentype::Minus | Tokentype::Multiply | Tokentype::Divide
            | Tokentype::Equal | Tokentype::EqualEqual | Tokentype::NotEqual
            | Tokentype::Less | Tokentype::Greater | Tokentype::LessEqual | Tokentype::GreaterEqual
            | Tokentype::And | Tokentype::Or | Tokentype::Not => {
                format!("{:15}", format!("{:?}", token_type)).yellow().to_string()
            }
            
            // Punctuation
            Tokentype::Semicolon | Tokentype::Comma | Tokentype::Colon | Tokentype::Arrow
            | Tokentype::LeftParen | Tokentype::RightParen | Tokentype::LeftBrace | Tokentype::RightBrace => {
                format!("{:15}", format!("{:?}", token_type)).bright_black().to_string()
            }
            
            // Special tokens
            Tokentype::Eof => {
                format!("{:15}", format!("{:?}", token_type)).bright_purple().to_string()
            }
            
            // Error tokens
            Tokentype::Invalid => {
                format!("{:15}", format!("{:?}", token_type)).red().bold().to_string()
            }
        }
    }

    /// Formats the lexeme with appropriate styling
    ///
    /// ### Arguments
    ///
    /// * `lexeme` - The lexeme to format
    /// * `token_type` - The token type for context
    ///
    /// ### Returns
    ///
    /// A formatted string
    fn format_lexeme(&self, lexeme: &str, token_type: &Tokentype) -> String {
        match token_type {
            Tokentype::StringLiteral => {
                format!("\"{}\"", lexeme).green().to_string()
            }
            Tokentype::Invalid => {
                format!("'{}'", lexeme).red().to_string()
            }
            Tokentype::Eof => {
                "<EOF>".bright_purple().to_string()
            }
            _ => {
                if lexeme.is_empty() {
                    "<empty>".dimmed().to_string()
                } else {
                    format!("'{}'", lexeme)
                }
            }
        }
    }
}

impl Default for TokenPrinter {
    fn default() -> Self {
        Self::new()
    }
}
