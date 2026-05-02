# Token Printer

A command-line tool for analyzing and printing tokens from Slang source code files.

## Overview

The Token Printer is a lexical analysis tool that tokenizes Slang source files and displays the resulting tokens in various formats. This tool is essential for:

- **Debugging lexical analysis** - See exactly how your source code is being tokenized
- **Understanding tokenization** - Visualize how the lexer breaks down source code
- **Compiler development** - Verify token generation during lexer development
- **Educational purposes** - Learn how programming languages process source code at the token level

## Features

- 🔤 **Complete tokenization** - Shows all tokens including whitespace and comments
- 🎨 **Colored output** - Enhanced readability with syntax highlighting
- 🔍 **Multiple formats** - Pretty-printed and debug formats available
- 🚀 **Fast processing** - Efficient tokenization using the Slang compilation pipeline
- 📍 **Position tracking** - Shows exact source positions for each token
- 🛡️ **Error handling** - Graceful handling of lexical errors

## Installation

From the project root directory:

```bash
cargo build --package token-printer
```

Or build all tools:

```bash
cargo build-tools  # Uses workspace alias
```

## Usage

### Basic Usage

```bash
# Pretty-print tokens (default format)
./target/debug/token-printer examples/hello_world.sl

# Use debug format
./target/debug/token-printer --format debug examples/program.sl
```

### Command Line Options

```
token-printer [OPTIONS] <FILE>

Arguments:
  <FILE>  Input source file to tokenize

Options:
  -f, --format <FORMAT>  Output format: pretty, debug [default: pretty]
  -h, --help             Print help
  -V, --version          Print version
```

### Output Formats

#### Pretty Format (Default)

```text
Tokenizing: examples/hello_world.sl

TOKEN           | VALUE                  | LINE:COL    | TYPE
----------------|------------------------|-------------|------------------
KEYWORD         | var                    | 1:1         | Keyword
IDENTIFIER      | message                | 1:5         | Identifier  
ASSIGN          | =                      | 1:13        | Operator
STRING_LITERAL  | "Hello, World!"        | 1:15        | Literal
SEMICOLON       | ;                      | 1:30        | Punctuation
IDENTIFIER      | print                  | 2:1         | Identifier
LEFT_PAREN      | (                      | 2:6         | Punctuation
IDENTIFIER      | message                | 2:7         | Identifier
RIGHT_PAREN     | )                      | 2:14        | Punctuation
SEMICOLON       | ;                      | 2:15        | Punctuation
EOF             |                        | 2:16        | Special
```

#### Debug Format

```text
Tokenizing: examples/hello_world.sl

Token { kind: Keyword(Var), lexeme: "var", line: 1, column: 1 }
Token { kind: Identifier, lexeme: "message", line: 1, column: 5 }
Token { kind: Assign, lexeme: "=", line: 1, column: 13 }
Token { kind: StringLiteral, lexeme: "\"Hello, World!\"", line: 1, column: 15 }
Token { kind: Semicolon, lexeme: ";", line: 1, column: 30 }
Token { kind: Identifier, lexeme: "print", line: 2, column: 1 }
Token { kind: LeftParen, lexeme: "(", line: 2, column: 6 }
Token { kind: Identifier, lexeme: "message", line: 2, column: 7 }
Token { kind: RightParen, lexeme: ")", line: 2, column: 14 }
Token { kind: Semicolon, lexeme: ";", line: 2, column: 15 }
Token { kind: Eof, lexeme: "", line: 2, column: 16 }
```

## Examples

### Analyze token structure

```bash
./target/debug/token-printer examples/variables.sl
```

### Debug tokenization issues

```bash
./target/debug/token-printer --format debug examples/problematic.sl
```

### Process multiple files

```bash
for file in examples/*.sl; do
    echo "=== $file ==="
    ./target/debug/token-printer "$file"
    echo
done
```

## Token Types

The tool recognizes various token types in Slang:

### Keywords
- `var`, `const`, `function`, `if`, `else`, `while`, `for`, `return`, etc.

### Identifiers
- Variable names, function names, user-defined identifiers

### Literals
- **String literals**: `"Hello, World!"`
- **Integer literals**: `42`, `0`, `-123`
- **Float literals**: `3.14`, `0.5`, `-2.7`
- **Boolean literals**: `true`, `false`

### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `&&`, `||`, `!`
- **Assignment**: `=`, `+=`, `-=`, etc.

### Punctuation
- **Delimiters**: `(`, `)`, `{`, `}`, `[`, `]`
- **Separators**: `,`, `;`, `:`
- **Other**: `.`, `->`, `::`, etc.

## Integration

### VS Code Tasks

Add to `.vscode/tasks.json`:

```json
{
    "label": "Print Tokens",
    "type": "shell",
    "command": "./target/debug/token-printer",
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
tokens: $(SOURCE_FILE)
	./target/debug/token-printer --format pretty $(SOURCE_FILE)

tokens-debug: $(SOURCE_FILE)
	./target/debug/token-printer --format debug $(SOURCE_FILE)
```

### Shell Scripts

Create a tokenize script:

```bash
#!/bin/bash
# tokenize.sh - Tokenize Slang source files

if [ $# -eq 0 ]; then
    echo "Usage: $0 <slang-file> [format]"
    exit 1
fi

FORMAT=${2:-pretty}
./target/debug/token-printer --format "$FORMAT" "$1"
```

## Development

### Building from Source

```bash
cd tools/token-printer
cargo build
```

### Running Tests

```bash
cd tools/token-printer
cargo test
```

### Dependencies

- `slang_compilation_pipeline` - Core compilation infrastructure
- `slang_frontend` - Tokenization and lexical analysis
- `clap` - Command-line argument parsing
- `colored` - Terminal color support

## Troubleshooting

### Common Issues

**File not found**: Ensure the input file path is correct and the file exists.

```bash
./target/debug/token-printer nonexistent.sl
# Error: No such file or directory (os error 2)
```

**Lexical errors**: The tool will report tokenization errors with position information.

```text
Lexical error at line 5, column 12: Unexpected character: '@'
```

**Empty files**: Empty files will only produce an EOF token.

```text
TOKEN | VALUE | LINE:COL | TYPE
------|-------|----------|------
EOF   |       | 1:1      | Special
```

### Performance Considerations

- Large files are processed efficiently using streaming tokenization
- Memory usage scales linearly with file size
- For very large files, consider processing in chunks

### Getting Help

- Use `--help` for command-line options
- Check the main Slang documentation for language syntax
- Refer to the lexer implementation for token definitions

## Use Cases

### Compiler Development

```bash
# Verify new keyword tokenization
echo 'async function test() {}' | ./target/debug/token-printer /dev/stdin

# Check operator precedence handling
./target/debug/token-printer examples/expressions.sl
```

### Language Learning

```bash
# Understand how complex expressions are tokenized
./target/debug/token-printer examples/math_expressions.sl

# See how string interpolation works
./target/debug/token-printer examples/string_templates.sl
```

### Debugging

```bash
# Find tokenization issues in failing code
./target/debug/token-printer --format debug problematic_code.sl

# Compare tokenization before and after language changes
diff <(./target/debug/token-printer old_syntax.sl) \
     <(./target/debug/token-printer new_syntax.sl)
```

## License

This tool is part of the Slang project and is licensed under GPL-3.0.
