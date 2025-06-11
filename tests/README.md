# End-to-End Tests

This directory contains end-to-end tests for the Slang programming language interpreter. These tests validate the complete compilation and execution pipeline, from source code parsing through bytecode generation to final program execution.

## Overview

The end-to-end tests ensure that the entire Slang interpreter works correctly by testing real programs through the complete compilation and execution process. Each test writes a temporary Slang source file, compiles it, and verifies the expected output or error behavior.

## Test Structure

The tests are organized into logical categories that mirror the language features:

### Core Test Modules

- **`syntax/`** - Basic language syntax and parsing tests
- **`functions/`** - Function definition, calling, and parameter handling
- **`operators/`** - All operator types (binary, unary, mathematical, logical, relational)
- **`statement/`** - Statement types (variable declarations, assignments)
- **`types/`** - Type system tests and error handling

### Test Utilities

The `test_utils.rs` module provides helper functions for running tests:

- **`execute_program_and_assert()`** - Executes a program and validates output
- **`execute_program_expect_error()`** - Tests error cases and validates error messages

## Test Categories

### Syntax Tests (`syntax/`)

Tests fundamental language constructs:

- **`basic.rs`** - Parentheses, operator precedence, nested blocks
- **`comments.rs`** - Comment parsing and handling
- **`errors.rs`** - Syntax error detection and reporting

### Function Tests (`functions/`)

Validates function-related features:

- **`function_basics.rs`** - Function definition, calling, parameters, return values
- **`error.rs`** - Function-related error cases (undefined functions, parameter mismatches)

### Operator Tests (`operators/`)

Comprehensive operator testing organized by type:

- **`binary/arithmetic/`** - Arithmetic operators (+, -, *, /, %)
- **`binary/logical/`** - Logical operators (&&, ||)
- **`binary/relational/`** - Comparison operators (<, >, ==, !=, <=, >=)
- **`unary/`** - Unary operators (-, !, etc.)

### Statement Tests (`statement/`)

Tests different statement types:

- **`assignment/`** - Variable assignment operations
- **`conditional/`** - Conditional statements and expressions (if/else)
- **`variable_declaration/`** - Variable declaration with and without initialization

### Type Tests (`types/`)

Type system validation:

- **`errors.rs`** - Type-related errors (undefined variables, unknown types)

## Running Tests

### Run All End-to-End Tests

```bash
cargo test --test e2e_tests
```

### Run Specific Test Categories

```bash
# Run only syntax tests
cargo test --test e2e_tests syntax

# Run only function tests
cargo test --test e2e_tests functions

# Run only operator tests
cargo test --test e2e_tests operators

# Run only statement tests
cargo test --test e2e_tests statement

# Run only conditional tests
cargo test --test e2e_tests conditional
```

### Run Individual Tests

```bash
# Run a specific test function
cargo test --test e2e_tests test_nested_blocks
```

## Test Workflow

Each test follows this general pattern:

1. **Setup**: Create a temporary directory and source file
2. **Execute**: Run the Slang interpreter with the test program
3. **Compile**: Generate bytecode from the source (for compilation tests)
4. **Run**: Execute the compiled bytecode (for runtime tests)
5. **Validate**: Assert expected output or error messages
6. **Cleanup**: Temporary files are automatically cleaned up

## Example Test

```rust
#[test]
fn with_multiple_params() {
    let program = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
        
        print_value(add(20, 22));
    "#;
    execute_program_and_assert(program, "42");
}
```

This test:

1. Defines a Slang program with a function that adds two numbers
2. Calls the function with arguments 20 and 22
3. Verifies that the pattern "42" occurs in stdout

## Error Testing

Error tests validate that the interpreter correctly detects and reports errors:

```rust
#[test]
fn undefined_variable() {
    let program = r#"
        print_value(y); 
    "#;
    execute_program_expect_error(program, "[E1234]" "Undefined variable: y");
}
```

This test:

1. Calls the print_value function with the undefined variable y
2. Executes the program and asserts that the output contains the specified error code "[E1234]"
3. Verifies that the pattern "Undefined variable: y" occurs in stderr

## Dependencies

The tests use the following crates:

- `assert_cmd` - For running the Slang binary and asserting on process behavior
- `predicates` - For flexible output matching
- `tempfile` - For creating temporary test files

## Integration with CI/CD

These end-to-end tests serve as the primary validation mechanism for the Slang compiler and should be run:

- Before merging any pull requests
- As part of the continuous integration pipeline
- When releasing new versions of the interpreter

The tests provide confidence that changes to any part of the interpreter (lexer, parser, semantic analyzer, compiler, or VM) don't break existing functionality.
