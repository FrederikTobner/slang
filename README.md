# Slang

[![Build and Test](https://github.com/FrederikTobner/slang/actions/workflows/build_and_test.yaml/badge.svg)](https://github.com/FrederikTobner/slang/actions/workflows/build_and_test.yaml)
[![codecov](https://codecov.io/gh/FrederikTobner/slang/graph/badge.svg?token=QDl7nyHWUn)](https://codecov.io/gh/FrederikTobner/slang)

Slang is a statically typed scripting language for learning purposes written in Rust.

## Overview

Slang is designed as an educational project to demonstrate language implementation concepts. It features:

- Static type checking
- Compilation to bytecode
- Execution via a virtual machine
- Interactive REPL
- Support for primitive types (integers, floats, booleans, strings, unit)
- Functions as first-class values with explicit type annotations
- Function type expressions (e.g., `fn(i32, string) -> bool`)

## Usage

Slang supports several modes of operation:

```bash
# Compile a Slang source file (.sl) to bytecode (.sip)
slang compile input.sl

# Execute a Slang source file directly
slang execute input.sl

# Run a compiled Slang bytecode file
slang run input.sip
```

## Language Syntax

For details about the language grammar, see [GRAMMAR.md](GRAMMAR.md).

### Debug Feature Flags

The following compilation flags can be used to debug the compiler.
