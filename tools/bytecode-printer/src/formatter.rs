use slang_backend::bytecode::Chunk;

pub mod formatters;

pub use formatters::{DebugFormatter, JsonFormatter, PrettyFormatter};

pub trait BytecodeFormatter {
    fn format(&self, chunk: &Chunk, name: &str) -> Result<String, Box<dyn std::error::Error>>;
}
