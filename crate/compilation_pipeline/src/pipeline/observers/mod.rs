// Generic type-safe observer system (primary system)
pub mod generic;

// Debug observers for monitoring compilation stages
pub mod debug;

// Re-export the new generic observer system as the primary API
pub use generic::{StageObserver, ObserverRegistry};

// Re-export type aliases for convenience
pub use generic::{TokenizationObserver, ParsingObserver, SemanticObserver, CodegenObserver};

// Re-export debug observers
pub use debug::{ASTPrintObserver, BytecodePrintObserver};
