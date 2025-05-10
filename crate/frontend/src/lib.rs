// Re-exported modules
pub mod lexer;
pub mod parser;
pub mod token;
pub mod type_checker;
pub mod error;

// Re-export common types
pub use lexer::tokenize;
pub use parser::Parser;
pub use token::{Token, Tokentype};
pub use type_checker::TypeChecker;