# Slang Compiler

This directory contains the main executable components for the Slang programming language compiler. 
It provides the command-line interface (CLI) and orchestrates the compilation pipeline from source code to bytecode execution.

## Overview

The Slang compiler is a complete interpreter that supports multiple execution modes:

- **Direct Execution** - Run source files directly without intermediate compilation
- **Compilation** - Compile source files to bytecode for later execution
- **Bytecode Execution** - Run pre-compiled bytecode files

## Command Line Interface

The CLI supports three main operation modes:

### Compile Mode

```bash
slang compile input.sl
```

Compiles source files to bytecode (.sip) format with optional optimization flags.

### Execute Mode

```bash
slang execute input.sl
```

Directly executes source files without generating intermediate bytecode files.

### Run Mode

```bash
slang run input.sip
```

