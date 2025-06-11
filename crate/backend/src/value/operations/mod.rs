pub mod arithmetic;
pub mod logical;
pub mod comparison;

// Re-export traits for convenience
pub use arithmetic::ArithmeticOps;
pub use logical::LogicalOps;
pub use comparison::ComparisonOps;

/// Combined trait for all value operations (for backward compatibility)
/// 
/// This trait automatically implements all value operations for any type
/// that implements the individual operation traits.
pub trait ValueOperation: ArithmeticOps + LogicalOps + ComparisonOps {}

/// Blanket implementation for any type that implements all three operation traits
impl<T> ValueOperation for T where T: ArithmeticOps + LogicalOps + ComparisonOps {}
