//! Parser module public API
//! 
//! This module provides a recursive descent parser for the Slang programming language.
//! It converts a stream of tokens into an Abstract Syntax Tree (AST) representation.
//! 
//! The parser is organized into several submodules:
//! - `core`: Main parser struct and coordination logic
//! - `error`: Error handling and reporting
//! - `expressions`: Expression parsing (binary, unary, literals, etc.)
//! - `literals`: Literal value parsing (integers, floats, strings)
//! - `statements`: Statement parsing (let, function declarations, etc.)
//! - `types`: Type parsing and validation
//! - `utilities`: Helper functions and common parsing utilities
//! 
//! # Example
//! 
//! ```rust
//! use slang_frontend::parser::Parser;
//! use slang_shared::CompilationContext;
//! 
//! let mut context = CompilationContext::new();
//! let mut parser = Parser::new(&tokens, &line_info, &mut context);
//! let statements = parser.parse()?;
//! ```

// Module declarations
mod core;
mod error;
mod expressions;
mod literals;
mod statements;
mod types;
mod utilities;

pub use core::Parser;