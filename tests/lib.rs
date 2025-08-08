mod cli;
mod codegen;
mod expression;
mod lexical;
mod statement;
mod syntax;
pub mod test_utils;
mod types;

// Re-export ErrorCode for use in tests
pub use slang_error::ErrorCode;

