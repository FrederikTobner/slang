pub mod operations;
pub mod type_system;
pub mod error;
pub mod semantic_analyzer; 
pub mod analyzer_modules;
pub mod visitors;
pub mod traits;
pub mod validation;

pub use error::{SemanticAnalysisError};
pub use analyzer_modules::CoreAnalyzer;
pub use semantic_analyzer::{execute, SemanticAnalyzer};