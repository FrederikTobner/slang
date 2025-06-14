# Slang Compiler

This directory contains the main executable components for the Slang programming language interpreter. It provides the command-line interface (CLI) and orchestrates the compilation pipeline from source code to bytecode execution.

## Overview

The Slang compiler is a complete interpreter that supports multiple execution modes:

- **Direct Execution** - Run source files directly without intermediate compilation
- **Compilation** - Compile source files to bytecode for later execution
- **Bytecode Execution** - Run pre-compiled bytecode files

## Architecture

The main application consists of several key modules:

### Core Modules

- **`main.rs`** - Application entry point and command routing
- **`cli.rs`** - Command-line interface implementation and compilation pipeline
- **`error.rs`** - Error handling and exit code management  
- **`exit.rs`** - Unix-style exit codes and program termination

## Compilation Pipeline

The compiler orchestrates a multi-stage compilation process:

1. **Lexical Analysis** - Tokenize source code
2. **Parsing** - Generate Abstract Syntax Tree
3. **Semantic Analysis** - Type checking and symbol resolution
4. **Code Generation** - Generate bytecode
5. **Execution** - Run bytecode using a VM

## Command-Line Interface

### Available Commands

#### Direct Execution

```bash
slang execute <source_file>
```

Compiles and runs a Slang source file directly without creating intermediate bytecode files.

#### Compilation

```bash
slang compile <source_file> [-o <output_file>]
```

Compiles a Slang source file to bytecode. If no output file is specified, uses the source filename with `.sip` extension.

#### Bytecode Execution

```bash
slang run <bytecode_file>
```

Executes a pre-compiled Slang bytecode file (`.sip` format).

### Examples

```bash
# Execute a source file directly
slang execute hello.sl

# Compile to bytecode
slang compile hello.sl -o hello.sip

# Run compiled bytecode
slang run hello.sip
```

## Bytecode Format

Compiled Slang programs are stored in `.sip` (Slang Intermediate Program) files using a compressed ZIP format:

- **Container**: ZIP archive with deflate compression
- **Content**: `bytecode.bin` file containing serialized bytecode
- **Benefits**: Compression reduces file size while maintaining portability

## Error Handling

The application uses a comprehensive error handling system with Unix-style exit codes:

### Error Types

- **I/O Errors** - File read/write issues with appropriate permission handling
- **ZIP Errors** - Bytecode archive creation/reading problems
- **Serialization Errors** - Bytecode serialization/deserialization failures
- **Compilation Errors** - Syntax, semantic, and code generation errors
- **Runtime Errors** - VM execution failures

### Exit Codes

The application follows Unix exit code conventions:

- `64` - Command line usage error
- `65` - Data format error
- `66` - Cannot open input file
- `70` - Internal software error
- `73` - Cannot create output file
- `74` - Input/output error
- `77` - Permission denied

## Integration with Crates

The main application integrates several specialized crates:

### Frontend Integration

- **`slang_frontend`** - Lexer, parser, and semantic analyzer
- **`slang_shared`** - Symbol table and type registry management

### Backend Integration  

- **`slang_backend`** - Bytecode compiler and virtual machine
- **`slang_ir`** - Abstract syntax tree and intermediate representation

### Type System

- **`slang_types`** - Type definitions and type checking utilities

## Development Features

### Debug Features

The application supports several debug features controlled by compile-time flags:

```rust
#[cfg(feature = "print-ast")]
// Prints the Abstract Syntax Tree for debugging

#[cfg(feature = "print-byte_code")]  
// Prints generated bytecode for inspection
```

### Platform Support

- **Cross-platform** - Supports Windows, macOS, and Linux
- **Windows Terminal** - Automatic virtual terminal enablement for colored output
- **Shell Integration** - Works with various shells (bash, zsh, PowerShell)

## File Extensions

- **`.sl`** - Slang source files
- **`.sip`** - Slang Intermediate Program (compiled bytecode)

## Dependencies

Key external dependencies:

- **`clap`** - Command-line argument parsing
- **`colored`** - Terminal color output
- **`zip`** - Bytecode file compression
- **`tempfile`** - Temporary file handling for tests

## Usage in Development

This module serves as the primary interface for:

- **End-to-end testing** - Integration tests use the CLI to validate complete workflows
- **User interaction** - Primary way users interact with the Slang language
- **Build pipeline** - Compilation and execution workflows for development
- **Debugging** - AST and bytecode inspection capabilities

## Error Reporting

The CLI provides comprehensive error reporting with:

- **Source location information** - Line and column numbers for errors
- **Colored output** - Visual distinction between error types
- **Context preservation** - Full source code context for error reporting
- **Graceful degradation** - Continues operation after recoverable errors
