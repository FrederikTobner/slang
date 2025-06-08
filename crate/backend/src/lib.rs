// Re-exported modules
pub mod bytecode;
pub mod codegen;
pub mod native;
pub mod value;
pub mod vm;

// Re-export common types
pub use vm::VM;
pub use codegen::CodeGenerator;
