# Slang Frontend

Frontend components of the Slang programming language.

## Purpose & Scope

The frontend crate is responsible for the initial stages of compilation, taking raw source code and transforming it into an Abstract Syntax Tree (AST) and performing semantic analysis. This includes:

- **Lexical Analysis**: Converting source text into tokens
- **Syntax Analysis**: Parsing tokens into an Abstract Syntax Tree (AST)
- **Semantic Analysis**: Type checking and validation

## Structure

The frontend consists of several key components:

- **Lexer (`lexer.rs`)**: Tokenizes source code, handling identifiers, keywords, literals, operators, and comments
- **Token Management (`token.rs`, `token_printer.rs`)**: Token definitions, source location tracking, and debugging utilities
- **Parser (`parser/`)**: Constructs an AST from tokens following the Slang grammar
  - **Core Parser (`parser/core.rs`)**: Main parsing logic and utilities
  - **Expression Parser (`parser/expressions.rs`)**: Handles expression parsing with precedence
  - **Statement Parser (`parser/statements.rs`)**: Parses all statement types
  - **Type Parser (`parser/types.rs`)**: Parses type expressions and annotations
  - **Literal Parser (`parser/literals.rs`)**: Handles literal value parsing
- **Semantic Analysis (`semantic_analysis/`)**: Performs static type analysis and validation
  - **Semantic Analyzer (`semantic_analysis/semantic_analyzer.rs`)**: Main analysis coordinator
  - **Type System (`semantic_analysis/type_system.rs`)**: Type checking and inference
  - **Error Collection (`semantic_analysis/error_collector.rs`)**: Error gathering and reporting
  - **Validation Modules (`semantic_analysis/validation/`)**: Specific validation passes
  - **Visitor Infrastructure (`semantic_analysis/visitors/`)**: AST traversal for analysis
- **Core Library (`lib.rs`)**: Exports the public API and coordinates components

## Integration with Other Crates

The frontend works closely with other crates in the Slang ecosystem:

- Uses the AST definitions from the `ir` crate
- Leverages the type system from the `types` crate
- Passes validated AST to the `backend` crate for compilation to bytecode

## Features

- Detailed error reporting with line and column information
- Visual error highlighting with red carets pointing to the error location
- Progressive error recovery to report multiple errors in a single pass
- Support for user-defined types and type checking

## Usage

The frontend is used internally by the main Slang CLI for:

- Compiling source files to bytecode
- Direct execution of Slang source files
