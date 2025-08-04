pub mod pretty;
pub mod debug;

pub use pretty::PrettyFormatter;
pub use debug::DebugFormatter;

use slang_frontend::Token;

/// Strategy trait for different token formatting approaches
pub trait TokenFormatter: Send + Sync {
    /// Format and print tokens with the given file name
    fn format_tokens(&self, tokens: &[Token], file_name: &str);
}
