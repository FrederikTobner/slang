// Re-exported modules
pub mod bytecode;
pub mod value;

// Re-export common types
pub use bytecode::{Chunk, OpCode};
pub use value::Value;