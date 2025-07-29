# Slang Crates

This directory contains the modular crate structure of the Slang programming language compiler and runtime. The architecture follows a layered approach with clear separation of concerns between different compilation phases and functionality.

## Crate Overview

```text
crate/
├── types/          # Type system foundation
├── error/          # Error handling and diagnostics  
├── derive/         # Procedural macros and code generation
├── ir/             # Intermediate representation (AST)
├── shared/         # Common utilities and data structures
├── frontend/       # Lexical analysis, parsing, and semantic analysis
└── backend/        # Code generation and runtime execution
```

## Architecture Flow

The crates are organized to support a clean compilation pipeline:

1. **`types`** - Provides the foundational type system used throughout compilation
2. **`error`** - Centralized error handling with rich diagnostic information
3. **`derive`** - Compile-time code generation via procedural macros
4. **`frontend`** - Transforms source code into validated AST (lexing → parsing → semantic analysis)
5. **`ir`** - Defines AST structures and visitor patterns for tree traversal
6. **`shared`** - Common functionality like diagnostics, symbol tables, and compilation context
7. **`backend`** - Generates bytecode from AST and provides the runtime execution engine

## Crate Details

### 🏗️ **types** - Type System Foundation

- Defines primitive and composite types (integers, floats, strings, functions, custom types)
- Type registry for managing type definitions
- Type inference and promotion operations
- **Dependencies**: None (foundation crate)

### 🚨 **error** - Error Handling & Diagnostics

- Structured error codes for compilation and runtime errors
- Rich error types with source location tracking
- Error formatting and user-friendly error messages
- **Dependencies**: None (foundation crate)

### ⚙️ **derive** - Procedural Macros

- `NamedEnum` - Generate name/from_str methods for enums
- `NumericEnum` - Automatic numeric value assignment
- `IterableEnum` - Make enums iterable over variants
- **Dependencies**: `proc-macro2`, `quote`, `syn`

### 🌳 **ir** - Intermediate Representation

- Abstract Syntax Tree (AST) node definitions
- Visitor pattern for AST traversal and transformation
- Factory methods for type-safe AST construction
- Location tracking for source mapping
- **Dependencies**: `types`, `error`

### 🔧 **shared** - Common Utilities

- Diagnostic engine for collecting and reporting errors
- Symbol table for variable and function management  
- Compilation context for global state
- **Dependencies**: `types`, `error`

### 📝 **frontend** - Lexical & Semantic Analysis

- **Lexer**: Converts source text into tokens with position tracking
- **Parser**: Builds AST from token stream with error recovery
- **Semantic Analyzer**: Type checking, scope resolution, and validation
- **Dependencies**: `types`, `error`, `ir`, `shared`

### ⚡ **backend** - Code Generation & Runtime

- **Code Generator**: Transforms AST into bytecode instructions
- **Virtual Machine**: Executes bytecode with stack-based evaluation
- **Native Functions**: Built-in operations and standard library
- **Dependencies**: `types`, `error`, `ir`, `shared`

## Design Principles

- **Separation of Concerns**: Each crate has a focused responsibility
- **Type Safety**: Extensive use of Rust's type system to prevent errors
- **Error Recovery**: Robust error handling with detailed diagnostics
- **Performance**: Zero-copy parsing and efficient AST representation
- **Extensibility**: Visitor pattern and factory methods for easy extension

## Inter-Crate Dependencies

```text
types ← error ← derive
  ↑       ↑
  ir ← shared ← frontend ← backend
```

The dependency graph ensures:

- Foundation crates (`types`, `error`) have no dependencies
- Each layer only depends on lower layers
- No circular dependencies between crates
- Clear compilation order and fast incremental builds

