# Slang IR

Intermediate Representation (IR) of the Slang programming language.

## Purpose & Scope

The IR crate defines the Abstract Syntax Tree (AST) and visitor pattern for traversing it. It provides:

- A common set of AST structures used by multiple components
- The visitor pattern for AST traversal
- Utilities for AST manipulation and inspection

## Structure

The IR consists of several key components:

- **AST Definitions (`ast.rs`)**: Defines the core Abstract Syntax Tree structures representing Slang programs
- **Visitor Pattern (`visitor.rs`)**: Implements the visitor design pattern for traversing the AST
- **AST Printer (`ast_printer.rs`)**: Debugging utility to visualize AST structures
- **Core Library (`lib.rs`)**: Exports the public API and connects the components

## Features

- Complete AST representation of Slang language constructs
- Location tracking for precise error reporting
- Visitor pattern for clean, extensible AST traversal
- Support for all Slang language features (expressions, statements, types)

## Usage

The IR is used:

- By the frontend during parsing to build AST representations
- By the type checker for semantic analysis
- By the backend compiler to generate bytecode
- For AST transformations and optimizations
- For debugging and visualization of program structure

## Integration

The IR crate serves as a central component in the Slang architecture:

- It defines structures used by the frontend parser
- It provides interfaces used by the type checker
- Its AST is consumed by the backend compiler
