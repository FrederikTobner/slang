# Slang Error

Error handling and error codes for the Slang programming language.

## Purpose & Scope

The error crate provides centralized error handling for the entire Slang compiler and runtime. It includes:

- **Error Codes**: Structured error codes for all compilation and runtime errors
- **Compiler Errors**: Rich error types with source location information
- **Error Formatting**: Beautiful error display with source code context
- **Error Reporting**: Utilities for collecting and reporting errors

## Structure

The error crate consists of:

- **Error Codes (`error_codes.rs`)**: Comprehensive error code enumeration
- **Compiler Error (`compiler_error.rs`)**: Main error type with location tracking
- **Error Utilities (`lib.rs`)**: Common utilities and type aliases

## Integration

This crate is used by:

- **Frontend**: For parse and semantic analysis errors
- **Backend**: For code generation errors
- **CLI**: For unified error reporting across all compilation phases
- **Tests**: For error validation in integration tests

## Features

- Source location tracking with line and column information
- Colored error output with visual highlighting
- Error code categorization for different compilation phases
- Context-aware error message formatting
- Support for error recovery and multiple error reporting
