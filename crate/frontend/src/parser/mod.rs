//! Parser module public API
//! 
//! This module provides a recursive descent parser for the Slang programming language.
//! It converts a stream of tokens into an Abstract Syntax Tree (AST) representation.
//! 
//! The parser is organized into several submodules:
//! - `core`: Main parser struct and coordination logic
//! - `error`: Error handling and reporting
//! - `expressions`: Expression parsing (binary, unary, literals, call expressions, conditionals, blocks)
//! - `literals`: Literal value parsing (integers, floats, strings)
//! - `statements`: Statement parsing (let, function declarations, etc.)
//! - `types`: Type parsing and validation
//! 
//! # Example
//! 
//! ```rust,no_run
//! use slang_frontend::parser::Parser;
//! use slang_shared::CompilationContext;
//! use slang_frontend::lexer::Lexer;
//! 
//! let source_code = "let x = 42;";
//! let lexer = Lexer::new(source_code);
//! let lexer_result = lexer.tokenize().unwrap();
//! let mut context = CompilationContext::new();
//! let mut parser = Parser::new(&lexer_result.tokens, &lexer_result.line_info, &mut context);
//! let statements = parser.parse().unwrap();
//! ```

// Module declarations
mod core;
mod error;
mod expressions;
mod literals;
mod statements;
mod types;

pub use core::{Parser, TokenPosition};