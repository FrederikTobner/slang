// Re-exported modules
pub mod bytecode;
pub mod value;
pub mod error;

// Re-export common types
pub use bytecode::{Chunk, OpCode};
pub use value::Value;