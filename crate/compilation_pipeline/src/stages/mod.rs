// Concrete implementations of compilation stages
pub mod tokenization;
pub mod parsing;
pub mod semantic;
pub mod codegen;

// Re-export all stages for convenience
pub use tokenization::TokenizationStage;
pub use parsing::ParsingStage;
pub use semantic::SemanticAnalysisStage;
pub use codegen::CodeGenerationStage;
