# Slang Types

Type system implementation for the Slang programming language.

## Purpose & Scope

The types crate defines the complete type system of Slang. It provides:

- Definitions for all built-in types
- A type registry for managing types
- Type checking utilities and validation
- Support for user-defined types (structs)

## Structure

The types crate consists of these key components:

- **Type Definitions (`types.rs`)**: Core type definitions including primitives and composite types
- **Type Registry**: Centralized system for registering and looking up types
- **Type Checking Logic**: Utilities for type compatibility and conversions
- **Core Library (`lib.rs`)**: Exports the public API and initializes the type system

## Features

- Rich type system with precise integer and floating-point types
- Support for various numeric types (i32, i64, u32, u64, f32, f64)
- Boolean and string type support
- User-defined struct types
- Type compatibility checking
- Value range validation for numeric types

## Usage

The types crate is used:
- By the frontend type checker for semantic analysis
- By the parser for early type recognition
- By the backend compiler for code generation
- For validating literal values against type constraints

## Integration

The types crate is utilized by multiple components:
- Frontend relies on it for type checking
- IR references it for type information in the AST
- Backend uses it during compilation for type-aware code generation