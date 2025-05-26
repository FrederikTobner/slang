# Compilation Context

The `slang_compilation_context` crate provides the core compilation infrastructure for the Slang programming language interpreter. It manages the compilation state, including type information and symbol resolution throughout the compilation process.

## Overview

This crate consists of two main components:

1. **CompilationContext** - The central compilation state manager that combines type registry and symbol table functionality
2. **SymbolTable** - A scoped symbol table for tracking variables, types, and functions during compilation

## Features

### CompilationContext

The `CompilationContext` struct serves as the primary interface for compilation state management. It provides:

- **Type Management**: Integration with the type registry for all primitive and custom types
- **Symbol Resolution**: Unified interface for defining and looking up symbols
- **Type Checking**: Helper methods for validating types and value ranges
- **Custom Type Registration**: Support for user-defined structs and other complex types

Key capabilities:

- Automatic registration of all primitive types (bool, integers, floats, string)
- Type compatibility checking and numeric type validation
- Symbol definition with conflict detection
- Integrated type and symbol lookup operations

### SymbolTable

The `SymbolTable` provides scoped symbol management with support for:

- **Symbol Kinds**: Differentiation between types, variables, and functions
- **Name Resolution**: Fast lookup by symbol name
- **Conflict Detection**: Prevention of duplicate symbol definitions
- **Type Association**: Each symbol maintains its associated type information

## Usage

### Basic Usage

```rust
use slang_compilation_context::{CompilationContext, SymbolKind};
use slang_types::{TypeId, TypeKind};

// Create a new compilation context
let mut context = CompilationContext::new();

// Define a variable symbol
let var_type_id = TypeId(0); // i32 type
context.define_symbol(
    "my_variable".to_string(),
    SymbolKind::Variable,
    var_type_id
).unwrap();

// Look up the symbol
let symbol = context.lookup_symbol("my_variable").unwrap();
println!("Found symbol: {} of type {}", symbol.name, context.get_type_name(&symbol.type_id));
```

### Type Checking

```rust
// Check if a type is numeric
if context.is_numeric_type(&type_id) {
    println!("Type supports arithmetic operations");
}

// Validate integer ranges
let value = 42i64;
if context.check_value_in_range(&value, &type_id) {
    println!("Value is within valid range for type");
}
```

### Custom Types

```rust
// Register a custom struct type
let fields = vec![
    ("name".to_string(), string_type_id),
    ("age".to_string(), i32_type_id),
];

let struct_type_id = context.register_struct_type(
    "Person".to_string(),
    fields
).unwrap();
```

## Symbol Kinds

The crate supports three kinds of symbols:

- **`SymbolKind::Type`** - Type definitions (primitives, structs, enums)
- **`SymbolKind::Variable`** - Variable declarations and parameters
- **`SymbolKind::Function`** - Function definitions and built-ins

## Dependencies

- `slang_types` - Provides the type system foundation including TypeId, TypeRegistry, and primitive type definitions

## Integration

This crate is designed to be used by:

- **Frontend**: For semantic analysis and type checking during parsing
- **Backend**: For code generation and runtime type information
- **Main compiler**: As the central compilation state manager

The compilation context serves as the bridge between the abstract syntax tree analysis phase and code generation, maintaining all necessary state for successful compilation.
