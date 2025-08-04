// Core traits and types for the new pipeline architecture
pub mod stage;
pub mod error;
pub mod result;
pub mod stages;
pub mod observers;

// Type-safe HList-based pipeline implementation
pub mod hlist;

// New hybrid pattern: execution chain + true builder
pub mod execution_chain;

// Chain-aware typed pipeline for compile-time observer validation
pub mod typed_builder;

// Re-export key types for convenient access
pub use hlist::{Execute, HCons, HList, HList1, HList2, HList3, HList4, HList5, HNil};

// Re-export new hybrid pattern types
pub use execution_chain::{ExecutionChain, ExecuteChain};

// Re-export typed builder components
pub use typed_builder::{
    ChainPipeline, TokenizationPipeline, ParsingPipeline, ASTPipeline, FullCompilationPipeline,
    HasStage, TokenizationStageMarker, ParsingStageMarker, SemanticStageMarker, CodegenStageMarker,
};
