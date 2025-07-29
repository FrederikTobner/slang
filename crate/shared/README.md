# Slang Shared

Shared utilities and infrastructure for the Slang programming language compiler.

## Purpose & Scope

The shared crate provides common functionality used across the frontend and backend compilation stages. It serves as the foundational layer for:

- **Compilation Context**: Global state management during compilation
- **Diagnostic Engine**: Error collection and reporting infrastructure  
- **Symbol Table**: Variable and function symbol management
- **Shared Data Structures**: Common types and utilities

## Structure

The shared crate consists of these key components:

- **Compilation Context (`compilation_context.rs`)**: Manages global compilation state and configuration
- **Diagnostic Engine (`diagnostic_engine.rs`)**: Collects, formats, and reports errors and warnings with suggestions
- **Symbol Table (`symbol_table.rs`)**: Tracks symbols (variables, functions) with scope management
- **Core Library (`lib.rs`)**: Exports the public API and type definitions

## Features

- **Centralized Error Handling**: Unified diagnostic collection across all compilation phases
- **Symbol Management**: Hierarchical symbol tables with scope resolution
- **Compilation State**: Global context for managing compilation parameters and state
- **Rich Diagnostics**: Detailed error reporting with suggestions and source location tracking

## Integration

This crate is used by:

- **Frontend**: For error reporting during lexing, parsing, and semantic analysis
- **Backend**: For code generation errors and symbol resolution
- **Main Compiler**: For orchestrating compilation phases and error reporting
- **Tests**: For validation of error handling and symbol resolution

## Public API

The crate exports the following main types:

```rust
pub use compilation_context::CompilationContext;
pub use diagnostic_engine::{Diagnostic, DiagnosticEngine, Suggestion};
pub use symbol_table::{Symbol, SymbolKind, SymbolTable};
```
