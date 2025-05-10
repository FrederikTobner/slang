# Slang Frontend

Frontend components of the Slang programming language.

## Purpose & Scope

The frontend crate is responsible for the initial stages of compilation, taking raw source code and transforming it into a form that can be further processed by the backend. This includes:

- **Lexical Analysis**: Converting source text into tokens
- **Syntax Analysis**: Parsing tokens into an Abstract Syntax Tree (AST)
- **Semantic Analysis**: Type checking and validation
- **IR Generation**: Compiling the AST to intermediate representation

## Structure

The frontend consists of several key components:

- **Lexer (`lexer.rs`)**: Tokenizes source code, handling identifiers, keywords, literals, operators, and comments (both single-line and multi-line)
- **Parser (`parser.rs`)**: Constructs an AST from tokens following the Slang grammar
- **AST Definitions (`ast.rs`)**: Defines the Abstract Syntax Tree structures
- **Type Checker (`type_checker.rs`)**: Performs static type analysis and validation
- **Error Handling (`error.rs`)**: Error collection and reporting system
- **Compiler (`compiler.rs`)**: Translates AST into bytecode
- **AST Printer (`ast_printer.rs`)**: Utility for debugging AST structures
- **Visitor Pattern (`visitor.rs`)**: Implements the visitor pattern for AST traversal
- **Type System (`types.rs`)**: Defines and manages the type system

## Usage

The frontend is used internally by the main Slang CLI for:
- Compiling source files to bytecode
- Interactive REPL operations
- Direct execution of Slang source files
