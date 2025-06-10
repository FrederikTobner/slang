pub mod lexer;
pub mod parser;
pub mod semantic_analysis;
pub mod token;
#[cfg(feature = "print-tokens")]
pub mod token_printer;

// Re-export error handling from slang_error
pub use slang_error::{ErrorCode, CompilerError, CompileResult, ErrorCollector, LineInfo, report_errors};
pub use lexer::tokenize;
pub use parser::Parser;
pub use semantic_analysis::{execute, SemanticAnalyzer};
pub use semantic_analysis::SemanticAnalysisError;
pub use token::{Token, Tokentype};
