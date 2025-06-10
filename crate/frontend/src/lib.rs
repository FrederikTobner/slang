pub mod lexer;
pub mod parser;
pub mod semantic_analyzer;
pub mod semantic_analysis;
pub mod semantic_error;
pub mod token;
#[cfg(feature = "print-tokens")]
pub mod token_printer;

// Re-export error handling from slang_error
pub use slang_error::{ErrorCode, CompilerError, CompileResult, ErrorCollector, LineInfo, report_errors};
pub use lexer::tokenize;
pub use parser::Parser;
pub use semantic_analyzer::SemanticAnalyzer;
pub use semantic_error::SemanticAnalysisError;
pub use token::{Token, Tokentype};
