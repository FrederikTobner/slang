mod expression;
mod statement;
mod lexical;
mod syntax;
pub mod test_utils;
mod types;
mod cli;
mod codegen;

// Re-export ErrorCode for use in tests
pub use slang_error::ErrorCode;