pub mod operations;
pub mod type_system;
pub mod error;
pub mod analyzer; 
pub mod visitors;
pub mod traits;
pub mod semantic_analyzer;

pub use error::{SemanticAnalysisError};
pub use analyzer::CoreAnalyzer;
pub use semantic_analyzer::{execute, SemanticAnalyzer};