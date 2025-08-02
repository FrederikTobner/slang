mod formatters {
    pub mod error;
    pub mod pretty;
    pub mod json;
    pub mod compact;
    
    pub use error::FormatError;
    pub use pretty::PrettyFormatter;
    pub use json::JsonFormatter;
    pub use compact::CompactFormatter;
    
    use slang_ir::ast::Statement;
    use std::error::Error;
    
    /// Trait for AST formatters
    pub trait AstFormatter {
        /// Format a list of AST statements
        fn format(&self, statements: &[Statement]) -> Result<String, Box<dyn Error>>;
    }
}

pub use formatters::{AstFormatter, PrettyFormatter, JsonFormatter, CompactFormatter};
