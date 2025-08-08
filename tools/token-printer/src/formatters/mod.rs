pub mod debug;
pub mod pretty;

pub use debug::DebugFormatter;
pub use pretty::PrettyFormatter;

use slang_frontend::Token;

/// Strategy trait for different token formatting approaches
pub trait TokenFormatter: Send + Sync {
    /// Format and print tokens with the given file name
    fn format_tokens(&self, tokens: &[Token], file_name: &str);
}
