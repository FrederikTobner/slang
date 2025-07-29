# Slang

[![Build and Test](https://github.com/FrederikTobner/slang/actions/workflows/build_and_test.yaml/badge.svg)](https://github.com/FrederikTobner/slang/actions/workflows/build_and_test.yaml)
[![codecov](https://codecov.io/gh/FrederikTobner/slang/graph/badge.svg?token=QDl7nyHWUn)](https://codecov.io/gh/FrederikTobner/slang)

Slang is a statically typed scripting language for learning purposes written in Rust.

## Overview

Slang is designed as an educational project to demonstrate language implementation concepts. It features:

- Static type checking
- Compilation to bytecode
- Execution via a virtual machine
- Support for primitive types (integers, floats, booleans, strings, unit)
- Functions as first-class values with explicit type annotations
- Function type expressions (e.g., `fn(i32, string) -> bool`)

## Usage

Slang supports several modes of operation:

```bash
# Compile a Slang source file (.sl) to bytecode (.sip)
slang compile input.sl

# Execute a Slang source file directly
slang execute input.sl

# Run a compiled Slang bytecode file
slang run input.sip
```

## Language Syntax

For details about the language grammar, see [GRAMMAR.md](GRAMMAR.md).

## Building

Build the project using Cargo:

```bash
# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Project Structure

The project is organized as a Rust workspace with multiple crates:

- **`crate/`** - Core language implementation crates
  - **`types/`** - Type system foundation
  - **`error/`** - Error handling and diagnostics
  - **`derive/`** - Procedural macros
  - **`ir/`** - Intermediate representation (AST)
  - **`shared/`** - Common utilities
  - **`frontend/`** - Lexing, parsing, and semantic analysis
  - **`backend/`** - Code generation and virtual machine
- **`src/`** - Main CLI application
- **`tests/`** - End-to-end integration tests
- **`benches/`** - Performance benchmarks

For detailed information about the crate structure, see [crate/README.md](crate/README.md).

## Features

The Slang language supports:

- **Static Type System**: Rich type checking with primitive and user-defined types
- **Functions**: First-class functions with explicit type annotations
- **Control Flow**: If/else expressions and statements
- **Variables**: Mutable and immutable variable declarations
- **Error Handling**: Comprehensive error reporting with source location tracking
- **Bytecode Compilation**: Efficient compilation to bytecode format
- **Virtual Machine**: Stack-based execution engine

## Development

The project uses modern Rust development practices:

- **Workspace Organization**: Modular crate structure for clean separation of concerns
- **Error Recovery**: Robust error handling throughout the compilation pipeline
- **Performance Testing**: Comprehensive benchmarking suite using Divan
- **Integration Testing**: End-to-end tests covering all language features
- **CI/CD**: Automated testing and code coverage with GitHub Actions

## License

This project is licensed under the GPL-3.0 License. See [LICENSE](LICENSE) for details.
