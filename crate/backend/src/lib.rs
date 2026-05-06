// Re-exported modules
pub mod artifact_file;
pub mod bytecode;
pub mod codegen;
pub mod native;
pub mod value;
pub mod vm;

// Re-export common types
pub use artifact_file::{SlangArtifactFile, SlangArtifactFileError};
pub use codegen::CodeGenerator;
pub use vm::VM;
