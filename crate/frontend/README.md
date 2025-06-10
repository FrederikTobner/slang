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
- **Parser (`parser.rs`)**: Constructs an AST from tokens following the Slang grammar
- **Semantic Analyzer (`semantic_analyzer.rs`)**: Performs static type analysis and validation
- **Error Handling (`error.rs`)**: Error collection and reporting system with source location tracking
- **Token Management (`token.rs`)**: Token definitions and source line tracking

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
