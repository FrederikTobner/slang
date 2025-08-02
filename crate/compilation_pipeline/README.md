# Slang Compilation Pipeline

A composable, multi-stage compilation infrastructure for the Slang programming language with comprehensive error recovery and diagnostic capabilities.

## Overview

The Slang Compilation Pipeline provides a robust, modular architecture for compiling Slang source code. It implements a stage-based compilation process where each stage can be individually configured, monitored, and tested. The pipeline supports both fail-fast and error recovery modes, making it suitable for both production compilation and development tools.

## Features

- 🏗️ **Composable Architecture** - Chain compilation stages together with clear boundaries
- 🛡️ **Error Recovery** - Continue compilation after errors to collect multiple issues in one pass
- 📊 **Rich Diagnostics** - Comprehensive error reporting with source context and position information
- 🔍 **Stage Monitoring** - Observer pattern for monitoring compilation progress and debugging
- ⚡ **Performance** - Optimized pipeline execution with minimal overhead
- 🧪 **Testability** - Each stage can be tested independently with custom inputs
- 🎯 **Type Safety** - Strong typing throughout the pipeline with compile-time stage validation

## Architecture

### Core Components

#### CompilationStage Trait
```rust
pub trait CompilationStage: Send + Sync {
    type Input: 'static;
    type Output: 'static;
    
    fn execute(&self, input: Self::Input, context: &mut StageContext, 
               diagnostics: &mut DiagnosticEngine) -> Result<Self::Output, ()>;
    fn name(&self) -> &'static str;
    fn is_critical(&self) -> bool { true }
}
```

#### Built-in Stages
- **TokenizationStage** - Converts source code to tokens
- **ParsingStage** - Transforms tokens into Abstract Syntax Tree (AST)
- **SemanticAnalysisStage** - Performs type checking and semantic validation
- **CodeGenerationStage** - Generates bytecode from validated AST

#### PipelineBuilder
Fluent API for constructing custom compilation pipelines:
```rust
let pipeline = PipelineBuilder::new(&source)
    .add_stage(TokenizationStage)
    .add_stage(ParsingStage)
    .with_error_strategy(ErrorStrategy::Recover { continue_on_non_critical: true })
    .with_file_name("example.sl".to_string())
    .build();
```

## Usage

### Basic Compilation

```rust
use slang_compilation_pipeline::{CompilationPipeline, CompilationResult};

let source = r#"
    var message = "Hello, World!";
    print(message);
"#;

let result = CompilationPipeline::new(source, Some("hello.sl".to_string()), false)
    .execute_all_stages();

match result {
    CompilationResult::Success { output, diagnostics } => {
        // Compilation succeeded - output contains bytecode
        println!("Compilation successful!");
        if diagnostics.has_warnings() {
            println!("Warnings: {}", diagnostics.warning_count());
        }
    }
    CompilationResult::Failed { diagnostics } => {
        // Handle compilation errors
        for error in diagnostics.errors() {
            eprintln!("Error: {}", error);
        }
    }
}
```

### Custom Pipeline Construction

```rust
use slang_compilation_pipeline::pipeline::{
    builder::PipelineBuilder,
    stages::*,
    error::ErrorStrategy,
};

// Create a custom pipeline with only tokenization and parsing
let pipeline = PipelineBuilder::new(&source)
    .add_stage(TokenizationStage)
    .add_stage(ParsingStage)
    .with_error_strategy(ErrorStrategy::Recover { 
        continue_on_non_critical: true 
    })
    .with_file_name("example.sl".to_string())
    .build();

let result = pipeline.execute();
```

### Stage Observers

Monitor compilation progress with type-safe observers:

```rust
use slang_compilation_pipeline::pipeline::observers::*;

struct TokenPrintObserver;

impl StageObserver<String, Vec<Token>> for TokenPrintObserver {
    fn on_stage_start(&self, input: &String) {
        println!("Starting tokenization of {} characters", input.len());
    }
    
    fn on_stage_success(&self, output: &Vec<Token>) {
        println!("Tokenization produced {} tokens", output.len());
    }
    
    fn on_stage_error(&self, error: &CompilerError) {
        eprintln!("Tokenization error: {}", error);
    }
}

let pipeline = PipelineBuilder::new(&source)
    .add_stage(TokenizationStage)
    .add_tokenization_observer(TokenPrintObserver)
    .build();
```

### Error Recovery Strategies

#### Fail Fast (Default)
```rust
let pipeline = PipelineBuilder::new(&source)
    .add_stage(TokenizationStage)
    .add_stage(ParsingStage)
    .with_error_strategy(ErrorStrategy::FailFast)
    .build();
```

#### Error Recovery
```rust
let pipeline = PipelineBuilder::new(&source)
    .add_stage(TokenizationStage)
    .add_stage(ParsingStage)
    .add_stage(SemanticAnalysisStage)
    .with_error_strategy(ErrorStrategy::Recover { 
        continue_on_non_critical: true 
    })
    .build();
```

## Compilation Stages

### 1. Tokenization Stage
**Input**: `String` (source code)  
**Output**: `Vec<Token>`

Converts source code into a stream of tokens, handling:
- Keywords and identifiers
- Literals (strings, numbers, booleans)
- Operators and punctuation
- Comments and whitespace
- Error recovery for invalid characters

### 2. Parsing Stage
**Input**: `Vec<Token>`  
**Output**: `Vec<Statement>` (AST)

Transforms tokens into an Abstract Syntax Tree:
- Recursive descent parsing
- Operator precedence handling
- Error recovery with synchronization points
- Rich error messages with context

### 3. Semantic Analysis Stage
**Input**: `Vec<Statement>` (AST)  
**Output**: `Vec<Statement>` (Analyzed AST)

Performs semantic validation:
- Type checking and inference
- Variable binding and scoping
- Function signature validation
- Dead code detection

### 4. Code Generation Stage
**Input**: `Vec<Statement>` (Analyzed AST)  
**Output**: `Chunk` (Bytecode)

Generates executable bytecode:
- Stack-based virtual machine instructions
- Constant pool management
- Jump instruction resolution
- Optimization passes

## Error Handling

### Diagnostic Engine Integration

The pipeline integrates seamlessly with the Slang diagnostic engine:

```rust
match result {
    CompilationResult::Success { output, diagnostics } => {
        // Check for warnings
        if diagnostics.has_warnings() {
            for warning in diagnostics.warnings() {
                println!("Warning: {}", warning);
            }
        }
    }
    CompilationResult::Failed { diagnostics } => {
        // Print all errors with source context
        for error in diagnostics.errors() {
            eprintln!("{}", error.format_with_source(&source));
        }
    }
}
```

### Error Recovery

Error recovery allows compilation to continue after non-fatal errors:

- **Lexical errors**: Skip invalid characters and continue tokenizing
- **Syntax errors**: Synchronize at statement boundaries
- **Semantic errors**: Continue with partial type information
- **Multiple error collection**: Report all issues in a single compilation pass

## Development Features

### Debug Features

Enable debug output during compilation:

```toml
[dependencies]
slang_compilation_pipeline = { path = "../compilation_pipeline", features = ["print-ast", "print-byte_code"] }
```

- `print-ast` - Display AST structure during parsing
- `print-byte_code` - Show generated bytecode during compilation

### Benchmarking

For performance testing and optimization:

```toml
[dependencies]
slang_compilation_pipeline = { path = "../compilation_pipeline", features = ["benchmarking"] }
```

Enables criterion-based benchmarking for each compilation stage.

## Testing

### Unit Testing Individual Stages

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use slang_compilation_pipeline::pipeline::stages::TokenizationStage;
    
    #[test]
    fn test_tokenization_stage() {
        let stage = TokenizationStage;
        let mut context = StageContext::new("test".to_string(), None);
        let mut diagnostics = DiagnosticEngine::new();
        
        let result = stage.execute("let x = 42;".to_string(), &mut context, &mut diagnostics);
        
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 6); // let, x, =, 42, ;, EOF
    }
}
```

### Integration Testing

```rust
#[test]
fn test_full_pipeline() {
    let source = "var x = 42; print(x);";
    let result = CompilationPipeline::new(source, None, false)
        .execute_all_stages();
    
    assert!(result.is_success());
    
    // Verify bytecode generation
    if let Some(chunk) = result.output() {
        let chunk = chunk.downcast::<Chunk>().unwrap();
        assert!(!chunk.instructions().is_empty());
    }
}
```

## Performance Considerations

### Stage Isolation
Each stage is isolated and stateless, enabling:
- **Parallel processing** of independent stages
- **Caching** of intermediate results
- **Incremental compilation** support

### Memory Management
- **Zero-copy** token representation where possible
- **Arena allocation** for AST nodes
- **Efficient string interning** for identifiers

### Optimization Strategies
- **Early termination** on critical errors
- **Lazy evaluation** of non-essential stages
- **Batched processing** for multiple files

## Dependencies

- **slang_shared** - Shared utilities and diagnostic engine
- **slang_frontend** - Tokenization and parsing components
- **slang_ir** - Intermediate representation and AST definitions
- **slang_backend** - Code generation and bytecode emission
- **colored** - Terminal output formatting
- **criterion** - Benchmarking framework (optional)

## Examples

### AST Printer Integration
```rust
// Extract AST without full compilation
let pipeline = PipelineBuilder::new(&source)
    .add_stage(TokenizationStage)
    .add_stage(ParsingStage)
    .build();

match pipeline.execute() {
    CompilationResult::Success { output, .. } => {
        let ast = output.downcast::<Vec<Statement>>().unwrap();
        println!("{:#?}", ast);
    }
    _ => eprintln!("Failed to parse"),
}
```

### Custom Error Reporter
```rust
struct CustomErrorReporter;

impl StageObserver<Vec<Token>, Vec<Statement>> for CustomErrorReporter {
    fn on_stage_error(&self, error: &CompilerError) {
        // Custom error formatting and reporting
        log::error!("Parsing failed: {}", error);
        metrics::increment_counter!("parse_errors");
    }
}
```

### Compiler Plugin System
```rust
// Create custom compilation stages
struct OptimizationStage;

impl CompilationStage for OptimizationStage {
    type Input = Vec<Statement>;
    type Output = Vec<Statement>;
    
    fn execute(&self, mut input: Self::Input, _context: &mut StageContext, 
               _diagnostics: &mut DiagnosticEngine) -> Result<Self::Output, ()> {
        // Perform AST optimizations
        optimize_dead_code(&mut input);
        optimize_constant_folding(&mut input);
        Ok(input)
    }
    
    fn name(&self) -> &'static str { "optimization" }
    fn is_critical(&self) -> bool { false }
}
```

## License

This crate is part of the Slang project and is licensed under GPL-3.0.
