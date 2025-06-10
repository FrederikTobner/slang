
pub mod arithmetic;
pub mod logical;
pub mod relational;
pub mod unary;
pub mod helpers;

// Re-export the main functions, but not the SemanticResult types to avoid conflicts
pub use arithmetic::{check_same_type_arithmetic, check_mixed_arithmetic_operation};
pub use logical::check_logical_operation;
pub use relational::check_relational_operation;
