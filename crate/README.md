# Slang Crates

This directory contains the modular crate structure of the Slang programming language compiler and runtime. The architecture follows a layered approach with clear separation of concerns between different compilation phases and functionality.

## Crate Overview

```text
crate/
├── types/          # Type system foundation
├── error/          # Error handling and diagnostics  
├── derive/         # Procedural macros and code generation
├── ir/             # Intermediate representation (AST)
├── shared/         # Common utilities and data structures
├── frontend/       # Lexical analysis, parsing, and semantic analysis
└── backend/        # Code generation and runtime execution
```

