# Slang

Slang is a statically typed scripting language for learning purposes written in Rust.

## Overview

Slang is designed as an educational project to demonstrate language implementation concepts. It features:

- Static type checking
- Compilation to bytecode
- Execution via a virtual machine
- Interactive REPL
- Support for primitive types (integers, floats, booleans, strings)
- Functions and structured programming
- Struct definitions

## Project Structure

The project is organized into the following crates:

- `frontend`: Handles lexing, parsing, AST construction, and type checking
- `ir`: Defines the intermediate representation (AST) and visitor pattern
- `types`: Contains type system definitions and utilities
- `backend`: Implements the compiler and virtual machine for executing bytecode
- `derive`: Provides procedural macros for code generation and boilerplate reduction

The codebase follows a modular architecture with clear separation of concerns:

```plaintext
slang/
├── crate/
│   ├── backend/    # Compilation to bytecode and VM execution
│   ├── derive/     # Procedural macros for code generation
│   ├── frontend/   # Lexer, parser, and type checker
│   ├── ir/         # AST definitions and visitors
│   └── types/      # Type system
├── src/            # CLI application
└── tests/          # End-to-end and integration tests
```

## Usage

Slang supports several modes of operation:

```bash
# Run the interactive REPL
slang repl

# Compile a Slang source file (.sl) to bytecode (.sip)
slang compile input.sl

# Execute a Slang source file directly
slang execute input.sl

# Run a compiled Slang bytecode file
slang run input.sip
```

## Design Principles

The project follows common software engineering principles:

- **Modularity**: Clear separation between components
- **Single Responsibility**: Each module has a focused purpose
- **Command Pattern**: CLI operations are encapsulated in command objects
- **Visitor Pattern**: AST traversal uses the visitor design pattern
- **Code Generation**: Procedural macros automate repetitive code patterns

## Procedural Macros

The `slang-derive` crate provides procedural macros that generate code for common patterns:

### TypeName Derive Macro

The `TypeName` derive macro automatically generates methods for converting between enum variants and their string representations:

```rust
use slang_derive::TypeName;

#[derive(TypeName)]
pub enum PrimitiveType {
    #[type_name = "i32"]
    I32,
    #[type_name = "bool"]
    Bool,
    String, // Uses lowercase variant name: "string"
}

// Generated methods:
// - type_name(&self) -> &'static str
// - from_str(s: &str) -> Option<Self>

const TYPE_NAME_I32: &str = PrimitiveType::I32.type_name(); // "i32"
```

### NumericEnum Derive Macro

The `NumericEnum` derive macro automatically generates methods for converting between enum variants and their numeric values:

```rust
use slang_derive::NumericEnum;

#[derive(NumericEnum)]
enum OpCode {
    Add = 1,         // Explicit value
    Subtract = 2,    // Explicit value
    Multiply,        // Implicit value: 3
    Divide,          // Implicit value: 4
}

// Generated method:
// - from_int<T: Into<usize>>(value: T) -> Option<Self>

let op = OpCode::from_int(1u8); // Some(OpCode::Add)
let op2 = OpCode::from_int(3); // Some(OpCode::Multiply)
```

See [crate/derive/README.md](crate/derive/README.md) for more details on available macros.

## Language Syntax

For details about the language grammar, see [GRAMMER.md](GRAMMER.md).

## Development

See [TODO.md](TODO.md) for planned improvements and future features.
