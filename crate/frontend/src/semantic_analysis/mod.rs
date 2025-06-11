pub mod analyzer_modules;
pub mod error;
pub mod error_collector;
pub mod operations;
pub mod semantic_analyzer;
pub mod traits;
pub mod type_system;
pub mod validation;
pub mod visitors;

pub use analyzer_modules::CoreAnalyzer;
pub use error::SemanticAnalysisError;
pub use error_collector::ErrorCollector;
pub use semantic_analyzer::execute;
pub use traits::SemanticResult;

