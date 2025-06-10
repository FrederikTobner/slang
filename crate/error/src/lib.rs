//! Error handling utilities for the Slang compiler
//!
//! This crate provides centralized error handling types and utilities used across
//! the Slang compiler infrastructure, including error codes, compiler errors,
//! and formatting utilities.

pub mod error_codes;
pub mod compiler_error;

pub use error_codes::ErrorCode;
pub use compiler_error::{CompilerError, CompileResult, ErrorCollector, LineInfo, report_errors};
