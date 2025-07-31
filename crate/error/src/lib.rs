//! Error handling utilities for the Slang compiler
//!
//! This crate provides centralized error handling types and utilities used across
//! the Slang compiler infrastructure, including error codes, compiler errors,
//! and formatting utilities.

pub mod error_codes;
pub mod compiler_error;
pub mod domain_error;
pub mod parse_error;
pub mod semantic_error;
pub mod codegen_error;
pub mod type_error;
pub mod parse_error_factory;

pub use error_codes::ErrorCode;
pub use compiler_error::{CompilerError, CompileResult, ErrorCollector, LineInfo, report_errors};
pub use domain_error::{DomainError, ErrorCategory, DomainResult};
pub use parse_error::{ParseError, ParseResult};
pub use semantic_error::{SemanticError, SemanticResult, FunctionCallErrorKind};
pub use codegen_error::{CodegenError, CodegenResult};
pub use type_error::{TypeError, TypeResult};
pub use parse_error_factory::ParseErrorFactory;
