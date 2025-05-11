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

The codebase follows a modular architecture with clear separation of concerns:

```
slang/
├── crate/
│   ├── backend/    # Compilation to bytecode and VM execution
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

## Language Syntax

For details about the language grammar, see [GRAMMER.md](GRAMMER.md).

## Development

See [TODO.md](TODO.md) for planned improvements and future features.

