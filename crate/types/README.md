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

- **Type Definitions (`types.rs`)**: Core type definitions including primitives, functions, and composite types
- **Type Registry (`registry.rs`)**: Centralized system for registering and looking up types by name or ID
- **Type Inference (`inference.rs`)**: Type promotion, operations, and compatibility checking utilities
- **Core Library (`lib.rs`)**: Exports the public API and initializes the type system

## Features

- Rich type system with precise integer and floating-point types
- Support for various numeric types (i32, i64, u32, u64, f32, f64)
- Boolean, string, and unit type support
- Function types with parameter and return type information
- User-defined struct types with field definitions
- Type registry for efficient type lookup and management
- Type inference and promotion system for compatibility checking
- Comprehensive type operation utilities

## Public API

The crate exports the following main types:

```rust
pub use registry::TypeRegistry;
pub use types::{FunctionType, PrimitiveType, StructType, TypeId, TypeInfo, TypeKind};
pub use inference::{TypePromotion, TypeQueries, TypeOperations};
```

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
