//! Error handling utilities for the Slang compiler
//!
//! This crate provides centralized error handling types and utilities used across
//! the Slang compiler infrastructure, including error codes, compiler errors,
//! and formatting utilities.

pub mod codegen_error;
pub mod compiler_error;
pub mod domain_error;
pub mod error_codes;
pub mod location;
pub mod parse_error;
pub mod parse_error_factory;
pub mod semantic_error;
pub mod type_error;

pub use codegen_error::{CodegenError, CodegenResult, ResourceType};
pub use compiler_error::{
    CompilationError, CompileResult, ErrorCollector, LineInfo, report_errors,
};
pub use domain_error::{DomainError, DomainResult, ErrorCategory};
pub use error_codes::ErrorCode;
pub use location::Location;
pub use parse_error::{ParseError, ParseResult};
pub use parse_error_factory::ParseErrorFactory;
pub use semantic_error::{FunctionCallErrorKind, SemanticError, SemanticResult};
pub use type_error::{TypeError, TypeResult};
