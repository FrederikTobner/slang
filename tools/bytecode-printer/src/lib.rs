//! # Bytecode Printer Library
//!
//! This library provides functionality for analyzing and formatting Slang bytecode.
//! It supports multiple output formats and integrates with the Slang compilation pipeline
//! to generate bytecode from source files.

pub mod cli;
pub mod format;
pub mod formatter;
pub mod observer;

// Re-export main functionality
pub use cli::{analyze_bytecode, Parser};
pub use format::BytecodeFormat;
pub use formatter::{BytecodeFormatter, PrettyFormatter, DebugFormatter, JsonFormatter};
