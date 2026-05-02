pub mod lexer;
pub mod parser;
pub mod semantic_analysis;
pub mod token;

// Re-export error handling from slang_error
pub use lexer::Lexer;
pub use semantic_analysis::SemanticAnalysisError;
pub use semantic_analysis::execute;
pub use slang_error::{
    CompilationError, CompileResult, ErrorCode, ErrorCollector, LineInfo, report_errors,
};
pub use token::{Token, Tokentype};
