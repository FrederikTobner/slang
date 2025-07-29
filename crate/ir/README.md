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
- **Location Tracking (`location.rs`)**: Source position tracking for precise error reporting
- **Factory Methods (`factory/`)**: Type-safe construction utilities for AST nodes
  - **Expressions (`factory/expressions.rs`)**: Factory methods for expression nodes
  - **Statements (`factory/statements.rs`)**: Factory methods for statement nodes
  - **Types (`factory/types.rs`)**: Factory methods for type expressions
  - **Locations (`factory/locations.rs`)**: Factory methods for location tracking
- **Core Library (`lib.rs`)**: Exports the public API and connects the components

## Features

- Complete AST representation of Slang language constructs
- Location tracking for precise error reporting and source mapping
- Visitor pattern for clean, extensible AST traversal and transformation
- Type-safe factory methods for constructing AST nodes
- Support for all Slang language features (expressions, statements, types)
- Rich debugging utilities for AST visualization

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
