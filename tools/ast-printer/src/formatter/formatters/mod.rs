pub mod pretty;
pub mod json;
pub mod compact;
pub mod error;

pub use pretty::PrettyFormatter;
pub use json::JsonFormatter;
pub use compact::CompactFormatter;
pub use error::FormatError;

/// Trait for different AST formatting strategies
pub trait AstFormatter {
    fn format_statements(&self, statements: &[slang_ir::ast::Statement]) -> String;
}
