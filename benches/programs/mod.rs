/// Program definitions for benchmarking
/// This module contains all the test programs used across different benchmarks

#[derive(Debug, Clone)]
pub struct BenchmarkProgram {
    pub name: &'static str,
    pub source: &'static str,
}

impl BenchmarkProgram {
    pub const fn new(name: &'static str, source: &'static str) -> Self {
        Self { name, source }
    }
}

// Re-export all program categories
pub mod core;
pub mod functions;
pub mod scopes;
pub mod types;
pub mod e2e;
pub mod vm;
pub mod errors;
pub mod templates;


// Consolidated arrays that combine programs from different categories
use core::{SIMPLE_EXPRESSION, NESTED_EXPRESSIONS, SIMPLE_ARITHMETIC, COMPLEX_ARITHMETIC, COMPLEX_EXPRESSIONS};
use e2e::{E2E_SIMPLE_ARITHMETIC, E2E_FIBONACCI_RECURSIVE, E2E_NESTED_SCOPES, E2E_FUNCTION_DEFINITIONS, E2E_CONTROL_FLOW};
use errors::{ERROR_MISSING_SEMICOLON, ERROR_UNMATCHED_PAREN, ERROR_INVALID_SYNTAX, ERROR_INCOMPLETE_FUNCTION, ERROR_INVALID_CHAR, ERROR_UNTERMINATED_STRING, ERROR_INVALID_NUMBER, ERROR_MIXED_ERRORS, ERROR_UNDEFINED_VARIABLE, ERROR_TYPE_MISMATCH, ERROR_UNDEFINED_FUNCTION, ERROR_SCOPE_ERROR, ERROR_RETURN_TYPE_MISMATCH, ERROR_PARAMETER_COUNT_MISMATCH};
use functions::{FUNCTION_DEFINITION, COMPLEX_FUNCTION, FUNCTION_CALLS, CONTROL_FLOW};
use scopes::{NESTED_SCOPES, SCOPE_RESOLUTION};
use vm::{
    VM_SIMPLE_ARITHMETIC, VM_FUNCTION_CALLS, VM_RECURSIVE_FIBONACCI, VM_NESTED_SCOPES,
    VM_INTEGER_ARITHMETIC, VM_FLOATING_POINT, VM_BOOLEAN_LOGIC, VM_STRING_OPERATIONS,
    VM_COMPARISON_OPERATIONS,
};
use types::{SIMPLE_TYPES, TYPE_CHECKING};

/// Consolidated array containing programs for parser performance testing
pub const PARSER_PROGRAMS: &[&BenchmarkProgram] = &[
    &SIMPLE_EXPRESSION,
    &NESTED_EXPRESSIONS,
    &COMPLEX_EXPRESSIONS,
    &FUNCTION_DEFINITION,
    &COMPLEX_FUNCTION,
];

/// Consolidated array containing programs for semantic analysis testing
pub const SEMANTIC_PROGRAMS: &[&BenchmarkProgram] = &[
    &SIMPLE_TYPES,
    &FUNCTION_CALLS,
    &TYPE_CHECKING,
    &SCOPE_RESOLUTION,
];

/// Consolidated array containing programs for codegen testing
pub const CODEGEN_PROGRAMS: &[&BenchmarkProgram] = &[
    &SIMPLE_ARITHMETIC,
    &COMPLEX_ARITHMETIC,
    &FUNCTION_CALLS,
    &CONTROL_FLOW,
    &NESTED_SCOPES,
];

/// Array containing programs for E2E testing
pub const E2E_PROGRAMS: &[&BenchmarkProgram] = &[
    &E2E_SIMPLE_ARITHMETIC,
    &E2E_FIBONACCI_RECURSIVE,
    &E2E_NESTED_SCOPES,
    &E2E_FUNCTION_DEFINITIONS,
    &E2E_CONTROL_FLOW,
];

/// Array containing core programs for codegen testing
pub const CODEGEN_CORE_PROGRAMS: &[&BenchmarkProgram] = &[
    &SIMPLE_ARITHMETIC,
    &COMPLEX_ARITHMETIC,
];

/// Array containing parser error cases
pub const PARSER_ERROR_PROGRAMS: &[&BenchmarkProgram] = &[
    &ERROR_MISSING_SEMICOLON,
    &ERROR_UNMATCHED_PAREN,
    &ERROR_INVALID_SYNTAX,
    &ERROR_INCOMPLETE_FUNCTION,
];

/// Array containing lexer error cases
pub const LEXER_ERROR_PROGRAMS: &[&BenchmarkProgram] = &[
    &ERROR_INVALID_CHAR,
    &ERROR_UNTERMINATED_STRING,
    &ERROR_INVALID_NUMBER,
    &ERROR_MIXED_ERRORS,
];

/// Array containing semantic error cases
pub const SEMANTIC_ERROR_PROGRAMS: &[&BenchmarkProgram] = &[
    &ERROR_UNDEFINED_VARIABLE,
    &ERROR_TYPE_MISMATCH,
    &ERROR_UNDEFINED_FUNCTION,
    &ERROR_SCOPE_ERROR,
    &ERROR_RETURN_TYPE_MISMATCH,
    &ERROR_PARAMETER_COUNT_MISMATCH,
];

/// Array containing programs for VM testing
pub const VM_PROGRAMS: &[&BenchmarkProgram] = &[
    &VM_SIMPLE_ARITHMETIC,
    &VM_FUNCTION_CALLS,
    &VM_RECURSIVE_FIBONACCI,
    &VM_NESTED_SCOPES,
];

/// Array containing VM value operation programs for testing different data types
pub const VM_VALUE_OPERATION_PROGRAMS: &[&BenchmarkProgram] = &[
    &VM_INTEGER_ARITHMETIC,
    &VM_FLOATING_POINT,
    &VM_BOOLEAN_LOGIC,
    &VM_STRING_OPERATIONS,
    &VM_COMPARISON_OPERATIONS,
];

/// Array containing scope programs for semantic analysis testing
pub const SCOPE_SEMANTIC_PROGRAMS: &[&BenchmarkProgram] = &[
    &SCOPE_RESOLUTION,
];

/// Array containing scope programs for codegen testing
pub const SCOPE_CODEGEN_PROGRAMS: &[&BenchmarkProgram] = &[
    &NESTED_SCOPES,
];
