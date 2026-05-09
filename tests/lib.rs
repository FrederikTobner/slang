pub mod assertions;
mod cli;
mod codegen;
mod expression;
mod lexical;
mod statement;
mod syntax;
mod types;

// Re-export ErrorCode for use in tests
pub use slang_error::ErrorCode;
