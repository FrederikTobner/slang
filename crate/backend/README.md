# Slang Backend

Backend execution engine of the Slang programming language.

## Purpose & Scope

The backend crate is responsible for executing Slang bytecode. It serves as the runtime for the language, providing:

- A virtual machine for bytecode execution
- Runtime error handling and reporting
- Memory management for Slang program execution
- Runtime value operations and manipulations

## Structure

The backend consists of two main components:

- **Virtual Machine (`vm.rs`)**: The core execution engine that interprets Slang bytecode, manages the execution stack, handles function calls, and performs operations on values
- **Core Library (`lib.rs`)**: Exports the public API and initializes the backend components

## Usage

The backend is used by the main Slang CLI for:
- Running compiled bytecode files (`.sip`)
- Executing Slang programs directly from source
- Powering the interactive REPL
- Testing language features and execution behavior

## Integration

The backend interfaces with:
- The IR crate to read and understand bytecode instructions
- The frontend indirectly through the compiled bytecode
