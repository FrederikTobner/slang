# AST Printer

A command-line tool for analyzing and printing Abstract Syntax Trees (ASTs) from Slang source code files.

## Overview

The AST Printer is a development tool that parses Slang source files and displays the resulting AST in various formats. This tool is particularly useful for:

- **Debugging parser behavior** - See exactly how your code is being parsed
- **Understanding code structure** - Visualize the hierarchical structure of your programs
- **Compiler development** - Analyze and verify AST generation during compiler development
- **Educational purposes** - Learn how programming languages are represented internally

## Features

- 🌳 **Multiple output formats**: Pretty-printed, JSON, and compact formats
- 🎨 **Colored output**: Enhanced readability with syntax highlighting
- 🔍 **Detailed analysis**: Shows complete AST structure with all nodes
- 🚀 **Fast processing**: Efficient parsing using the Slang compilation pipeline
- 📊 **Progress reporting**: Verbose mode shows compilation stages
- ⚡ **Error recovery**: Continues processing even with syntax errors when possible

## Installation

From the project root directory:

```bash
cargo build --package ast-printer
```

Or build all tools:

```bash
cargo build-tools  # Uses workspace alias
```

## Usage

### Basic Usage

```bash
# Pretty-print AST (default format)
./target/debug/ast-printer examples/hello_world.sl

# Specify output format
./target/debug/ast-printer --format json examples/program.sl
./target/debug/ast-printer --format compact examples/program.sl
```

### Command Line Options

```
ast-printer [OPTIONS] <FILE>

Arguments:
  <FILE>  Input source file to parse

Options:
  -f, --format <FORMAT>  Output format: pretty, json, compact [default: pretty]
      --semantic         Run semantic analysis and show the analyzed AST
  -v, --verbose          Show detailed compilation pipeline progress
  -h, --help             Print help
  -V, --version          Print version
```

### Output Formats

#### Pretty Format (Default)
```
Info: Parsing examples/hello_world.sl
Success: Successfully parsed 3 statements

Program
├── VariableDeclaration
│   ├── Identifier: "message"
│   └── StringLiteral: "Hello, World!"
├── FunctionCall
│   ├── Identifier: "print"
│   └── Arguments
│       └── Identifier: "message"
└── Return
    └── IntegerLiteral: 0
```

#### JSON Format
```json
{
  "type": "Program",
  "statements": [
    {
      "type": "VariableDeclaration",
      "identifier": "message",
      "initializer": {
        "type": "StringLiteral",
        "value": "Hello, World!"
      }
    }
  ]
}
```

#### Compact Format
```
Program[VariableDeclaration(message="Hello, World!"), FunctionCall(print, message), Return(0)]
```

## Examples

### Analyze a simple program
```bash
./target/debug/ast-printer examples/variables.sl
```

### Export AST as JSON for further processing
```bash
./target/debug/ast-printer --format json examples/functions.sl > ast_output.json
```

### Verbose mode for debugging
```bash
./target/debug/ast-printer --verbose examples/complex_program.sl
```

### Include semantic analysis
```bash
./target/debug/ast-printer --semantic examples/typed_program.sl
```

## Integration

The AST Printer can be integrated into development workflows:

### VS Code Tasks
Add to `.vscode/tasks.json`:
```json
{
    "label": "Print AST",
    "type": "shell",
    "command": "./target/debug/ast-printer",
    "args": ["${file}"],
    "group": "build",
    "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "shared"
    }
}
```

### Makefile Integration
```makefile
ast: $(SOURCE_FILE)
	./target/debug/ast-printer --format pretty $(SOURCE_FILE)

ast-json: $(SOURCE_FILE)
	./target/debug/ast-printer --format json $(SOURCE_FILE)
```

## Development

### Building from Source
```bash
cd tools/ast-printer
cargo build
```

### Running Tests
```bash
cd tools/ast-printer
cargo test
```

### Dependencies
- `slang_compilation_pipeline` - Core compilation infrastructure
- `slang_ir` - Intermediate representation and AST definitions
- `clap` - Command-line argument parsing
- `colored` - Terminal color support
- `serde_json` - JSON serialization

## Troubleshooting

### Common Issues

**File not found**: Ensure the input file path is correct and the file exists.
```bash
./target/debug/ast-printer nonexistent.sl
# Error: No such file or directory (os error 2)
```

**Syntax errors**: The tool will attempt to parse even with syntax errors, but may produce incomplete ASTs.
```bash
./target/debug/ast-printer broken_syntax.sl
# Error: Failed to parse source code. Check source file for syntax errors.
```

**Permission denied**: Ensure you have read permissions for the input file.

### Getting Help

- Use `--help` for command-line options
- Use `--verbose` to see detailed parsing steps
- Check the main Slang documentation for language syntax

## License

This tool is part of the Slang project and is licensed under GPL-3.0.
