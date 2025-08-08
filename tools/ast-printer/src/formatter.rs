mod formatters {
    pub mod compact;
    pub mod error;
    pub mod json;
    pub mod pretty;

    pub use compact::CompactFormatter;
    pub use error::FormatError;
    pub use json::JsonFormatter;
    pub use pretty::PrettyFormatter;

    use slang_ir::ast::Statement;
    use std::error::Error;

    /// Trait for AST formatters
    pub trait AstFormatter {
        /// Format a list of AST statements
        fn format(&self, statements: &[Statement]) -> Result<String, Box<dyn Error>>;
    }
}

pub use formatters::{AstFormatter, CompactFormatter, JsonFormatter, PrettyFormatter};
