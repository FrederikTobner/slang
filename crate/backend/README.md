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

