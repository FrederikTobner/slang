// Generic type-safe observer system (primary system)
pub mod generic;

// Re-export the new generic observer system as the primary API
pub use generic::{StageObserver, ObserverRegistry};

// Re-export type aliases for convenience
pub use generic::{TokenizationObserver, ParsingObserver, SemanticObserver, CodegenObserver};
