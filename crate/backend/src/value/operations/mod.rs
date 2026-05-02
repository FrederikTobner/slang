pub mod arithmetic;
pub mod comparison;
pub mod logical;

// Re-export traits for convenience
pub use arithmetic::ArithmeticOps;
pub use comparison::ComparisonOps;
pub use logical::LogicalOps;

/// Combined trait for all value operations (for backward compatibility)
///
/// This trait automatically implements all value operations for any type
/// that implements the individual operation traits.
pub trait ValueOperation: ArithmeticOps + LogicalOps + ComparisonOps {}

/// Blanket implementation for any type that implements all three operation traits
impl<T> ValueOperation for T where T: ArithmeticOps + LogicalOps + ComparisonOps {}
