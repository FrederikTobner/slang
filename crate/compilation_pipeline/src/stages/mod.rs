// Concrete implementations of compilation stages
pub mod codegen;
pub mod parsing;
pub mod semantic;
pub mod tokenization;

// Re-export all stages for convenience
pub use codegen::CodeGenerationStage;
pub use parsing::ParsingStage;
pub use semantic::SemanticAnalysisStage;
pub use tokenization::TokenizationStage;
