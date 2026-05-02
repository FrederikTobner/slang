# Bytecode Printer

A command-line tool for analyzing and printing bytecode from Slang source code files.

## Overview

The Bytecode Printer is a low-level analysis tool that compiles Slang source files to bytecode and displays the resulting virtual machine instructions in various formats. This tool is essential for:

- **Debugging code generation** - See exactly how your source code is compiled to bytecode
- **Understanding the virtual machine** - Visualize the stack-based instruction set
- **Compiler optimization analysis** - Analyze the efficiency of generated code
- **Educational purposes** - Learn how high-level constructs map to low-level instructions

## Features

- 🔧 **Complete compilation pipeline** - Full compilation from source to bytecode
- 🎨 **Multiple output formats** - Pretty-printed, debug, and JSON formats
- 📊 **Detailed instruction analysis** - Shows opcodes, operands, and values
- 🔍 **Constants and identifiers tables** - Complete symbol information
- 📍 **Source line mapping** - Links bytecode instructions to source lines
- 🚀 **Fast processing** - Efficient compilation using the Slang pipeline
- 🛡️ **Error handling** - Graceful handling of compilation errors

## Installation

From the project root directory:

```bash
cargo build --package bytecode-printer
```

Or build all tools:

```bash
cargo build-tools  # Uses workspace alias
```

## Usage

### Basic Usage

```bash
# Pretty-print bytecode (default format)
./target/debug/bytecode-printer examples/hello_world.sl

# Use debug format for detailed analysis
./target/debug/bytecode-printer --format debug examples/program.sl

# Export as JSON for automated analysis
./target/debug/bytecode-printer --format json examples/program.sl > bytecode.json
```

### Command Line Options

```
bytecode-printer [OPTIONS] <FILE>

Arguments:
  <FILE>  Input source file to compile to bytecode

Options:
  -f, --format <FORMAT>      Output format: pretty, debug, json [default: pretty]
      --chunk-name <NAME>    Custom name for the bytecode chunk
  -v, --verbose              Show detailed compilation pipeline progress
  -h, --help                 Print help
  -V, --version              Print version
```

### Output Formats

#### Pretty Format (Default)

```text
== hello_world ==
Offset | Line | Instruction    | Operand | Value
-------|------|----------------|---------|------------------
000000 |    1 | CONSTANT       | 0       | "Hello, World!"
000002 |    1 | SET_VARIABLE   | 0       | message
000004 |    2 | GET_VARIABLE   | 0       | message
000006 |    2 | PRINT          | -       | 
000007 |    3 | CONSTANT       | 1       | 0
000009 |    3 | RETURN         | -       | 

=== Constants ===
   0: "Hello, World!"
   1: 0

=== Identifiers ===
   0: message
```

#### Debug Format

```text
=== BYTECODE CHUNK: hello_world ===
Code size: 10 bytes
Constants: 2
Identifiers: 1
Lines: 10

=== RAW BYTECODE ===
0000: 00 00 01 00 08 00 09 00 00 01 

=== DISASSEMBLED INSTRUCTIONS ===
0000: 00 Constant 0 (line 1)
0002: 09 SetVariable 0 (line 1)
0004: 08 GetVariable 0 (line 2)
0006: 07 Print (line 2)
0007: 00 Constant 1 (line 3)
0009: 06 Return (line 3)

=== CONSTANTS TABLE ===
   0: String("Hello, World!")
   1: I32(0)

=== IDENTIFIERS TABLE ===
   0: "message"
```

#### JSON Format

```json
{
  "name": "hello_world",
  "statistics": {
    "code_size": 10,
    "constants_count": 2,
    "identifiers_count": 1,
    "lines_count": 10
  },
  "instructions": [
    {
      "offset": 0,
      "line": 1,
      "opcode": "CONSTANT",
      "operand": 0,
      "value": "\"Hello, World!\""
    },
    {
      "offset": 2,
      "line": 1,
      "opcode": "SET_VARIABLE",
      "operand": 0,
      "identifier": "message"
    },
    {
      "offset": 4,
      "line": 2,
      "opcode": "GET_VARIABLE",
      "operand": 0,
      "identifier": "message"
    }
  ],
  "constants": [
    {
      "index": 0,
      "value": "\"Hello, World!\"",
      "type": "string"
    }
  ],
  "identifiers": [
    {
      "index": 0,
      "name": "message"
    }
  ]
}
```

## Instruction Set

The Slang virtual machine uses a stack-based instruction set:

### Stack Operations
- **CONSTANT** - Push constant value onto stack
- **POP** - Remove top stack value
- **GET_VARIABLE** - Push variable value onto stack
- **SET_VARIABLE** - Set variable to top stack value

### Arithmetic Operations
- **ADD** - Add top two stack values
- **SUBTRACT** - Subtract top from second stack value
- **MULTIPLY** - Multiply top two stack values
- **DIVIDE** - Divide second by top stack value
- **NEGATE** - Negate top stack value

### Comparison Operations
- **EQUAL** / **NOT_EQUAL** - Equality comparison
- **GREATER** / **LESS** - Relational comparison
- **GREATER_EQUAL** / **LESS_EQUAL** - Relational comparison

### Logical Operations
- **BOOL_NOT** - Logical NOT
- **BOOL_AND** - Logical AND
- **BOOL_OR** - Logical OR

### Control Flow
- **JUMP** - Unconditional jump
- **JUMP_IF_FALSE** - Conditional jump
- **CALL** - Function call
- **RETURN** - Return from function

### Scope Management
- **BEGIN_SCOPE** - Start new variable scope
- **END_SCOPE** - End current variable scope

### I/O Operations
- **PRINT** - Print top stack value

## Examples

### Analyze simple expressions

```bash
./target/debug/bytecode-printer examples/math.sl
```

### Debug function calls

```bash
./target/debug/bytecode-printer --verbose examples/functions.sl
```

### Export for automated analysis

```bash
./target/debug/bytecode-printer --format json examples/complex.sl | jq '.instructions | length'
```

### Custom chunk naming

```bash
./target/debug/bytecode-printer --chunk-name "my_program" examples/app.sl
```

## Integration

### VS Code Tasks

Add to `.vscode/tasks.json`:

```json
{
    "label": "Print Bytecode",
    "type": "shell",
    "command": "./target/debug/bytecode-printer",
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
bytecode: $(SOURCE_FILE)
	./target/debug/bytecode-printer --format pretty $(SOURCE_FILE)

bytecode-debug: $(SOURCE_FILE)
	./target/debug/bytecode-printer --format debug $(SOURCE_FILE)

bytecode-json: $(SOURCE_FILE)
	./target/debug/bytecode-printer --format json $(SOURCE_FILE)
```

### Shell Scripts

Create a bytecode analysis script:

```bash
#!/bin/bash
# analyze_bytecode.sh - Analyze Slang bytecode

if [ $# -eq 0 ]; then
    echo "Usage: $0 <slang-file> [format]"
    exit 1
fi

FORMAT=${2:-pretty}
./target/debug/bytecode-printer --format "$FORMAT" --verbose "$1"
```

## Development

### Building from Source

```bash
cd tools/bytecode-printer
cargo build
```

### Running Tests

```bash
cd tools/bytecode-printer
cargo test
```

### Dependencies

- **slang_compilation_pipeline** - Full compilation infrastructure
- **slang_backend** - Bytecode generation and virtual machine
- **clap** - Command-line argument parsing
- **colored** - Terminal color support
- **serde_json** - JSON serialization

## Troubleshooting

### Common Issues

**Compilation errors**: The tool requires valid Slang source code that compiles successfully.

```bash
./target/debug/bytecode-printer broken_syntax.sl
# Error: Failed to compile source code to bytecode. Check source file for errors.
```

**File not found**: Ensure the input file path is correct.

```bash
./target/debug/bytecode-printer nonexistent.sl
# Error: Input file 'nonexistent.sl' does not exist
```

**Large bytecode output**: Use JSON format for programmatic processing of large bytecode.

```bash
./target/debug/bytecode-printer --format json large_program.sl | jq '.statistics'
```

### Performance Considerations

- **Memory usage** scales with program complexity and constant pool size
- **Large programs** benefit from JSON format for automated analysis
- **Verbose mode** adds compilation time but provides useful debugging information

### Getting Help

- Use `--help` for command-line options
- Use `--verbose` to see detailed compilation steps
- Check the main Slang documentation for language syntax
- Refer to the virtual machine documentation for instruction semantics

## Use Cases

### Compiler Development

```bash
# Verify instruction generation for new language features
./target/debug/bytecode-printer --format debug new_feature.sl

# Compare bytecode before and after optimizations
diff <(./target/debug/bytecode-printer old_codegen.sl) \
     <(./target/debug/bytecode-printer new_codegen.sl)
```

### Performance Analysis

```bash
# Count instructions in different implementations
./target/debug/bytecode-printer --format json algorithm1.sl | jq '.instructions | length'
./target/debug/bytecode-printer --format json algorithm2.sl | jq '.instructions | length'

# Analyze constant pool usage
./target/debug/bytecode-printer --format json program.sl | jq '.constants | group_by(.type) | map({type: .[0].type, count: length})'
```

### Educational Use

```bash
# Show how control structures compile
./target/debug/bytecode-printer examples/if_statement.sl
./target/debug/bytecode-printer examples/while_loop.sl

# Demonstrate function call overhead
./target/debug/bytecode-printer examples/recursive_function.sl
```

### Debugging

```bash
# Trace execution flow through bytecode
./target/debug/bytecode-printer --format debug problematic_code.sl

# Verify variable scoping in bytecode
./target/debug/bytecode-printer --verbose scoping_test.sl
```

## Advanced Features

### JSON Processing with jq

Extract specific information from bytecode:

```bash
# Get all string constants
./target/debug/bytecode-printer --format json program.sl | jq '.constants[] | select(.type == "string")'

# Find all function calls
./target/debug/bytecode-printer --format json program.sl | jq '.instructions[] | select(.opcode == "CALL")'

# Count instruction types
./target/debug/bytecode-printer --format json program.sl | jq '.instructions | group_by(.opcode) | map({opcode: .[0].opcode, count: length})'
```

### Automated Testing

Use in test scripts to verify code generation:

```bash
# Test that simple addition generates expected instructions
BYTECODE=$(./target/debug/bytecode-printer --format json simple_add.sl)
INSTRUCTION_COUNT=$(echo "$BYTECODE" | jq '.instructions | length')
if [ "$INSTRUCTION_COUNT" -ne 5 ]; then
    echo "ERROR: Expected 5 instructions, got $INSTRUCTION_COUNT"
    exit 1
fi
```

## License

This tool is part of the Slang project and is licensed under GPL-3.0.
