# Slang Backend

Backend compilation and execution engine of the Slang programming language.

## Purpose & Scope

The backend crate is responsible for compiling the AST to bytecode and executing Slang bytecode. It serves as both the compiler and runtime for the language, providing:

- Bytecode compilation from AST
- Bytecode serialization and deserialization
- A virtual machine for bytecode execution
- Runtime error handling and reporting
- Memory management for Slang program execution
- Runtime value operations and manipulations

## Structure

The backend consists of several key components:

- **CodeGenerator (`codegen.rs`)**: Translates AST into bytecode instructions
- **Virtual Machine (`vm.rs`)**: The core execution engine that interprets Slang bytecode
- **Bytecode (`bytecode.rs`)**: Definitions of bytecode instructions and chunks
- **Value System (`value.rs`)**: Runtime value representations and operations
- **Core Library (`lib.rs`)**: Exports the public API and initializes the backend components

## Features

- Stack-based virtual machine with efficient instruction dispatch
- Bytecode serialization for storing compiled programs
- Rich runtime value system supporting all Slang types
- Detailed runtime error reporting
- Support for native function calls

## Usage

The backend is used by the main Slang CLI for:

- Compiling AST to bytecode files (`.sip`)
- Running compiled bytecode files
- Executing Slang programs directly from source
- Powering the interactive REPL

## Integration

The backend interfaces with:

- The IR crate to process AST structures
- The types crate for type information during compilation
- The frontend indirectly through the AST
