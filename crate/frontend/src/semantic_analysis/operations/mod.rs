pub mod arithmetic;
pub mod helpers;
pub mod logical;
pub mod relational;
pub mod unary;

// Re-export the main functions, but not the SemanticResult types to avoid conflicts
pub use arithmetic::{check_mixed_arithmetic_operation, check_same_type_arithmetic};
pub use logical::check_logical_operation;
pub use relational::check_relational_operation;
