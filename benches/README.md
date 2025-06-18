# Performance Testing Suite for Slang Compiler

This directory contains a streamlined performance testing suite for the Slang compiler using [Criterion.rs](https://github.com/bheisler/criterion.rs) for comprehensive benchmarking and performance analysis.

## Overview

The performance testing suite provides:

- **Direct Criterion Integration**: Uses Criterion's built-in statistical analysis and reporting
- **Compilation Pipeline Testing**: Direct usage of `CompilationPipeline` and `compilation_pipeline` APIs
- **Comprehensive Coverage**: All major compiler stages from lexing to execution
- **Clean Architecture**: Minimal abstractions, maximum clarity
- **Professional Reports**: HTML reports with graphs and statistical analysis

## Architecture

### Benchmark Files

- **`lexer_benchmarks.rs`** - Tokenization performance across different input sizes and error cases
- **`parser_benchmarks.rs`** - AST generation and parsing performance with scalability testing  
- **`semantic_benchmarks.rs`** - Type checking and semantic analysis performance
- **`vm_benchmarks.rs`** - Bytecode execution and VM performance testing
- **`codegen_benchmarks.rs`** - Code generation and compilation performance
- **`e2e_benchmarks.rs`** - End-to-end compilation pipeline benchmarks

### Common Utilities (`common/` directory)

- **`program_builder.rs`** - Test program generation with controlled complexity
- **`mod.rs`** - Module organization for common utilities

**Key Design Principle**: All benchmarks use `CompilationPipeline` and `compilation_pipeline` directly - no abstraction layers.

## Quick Start

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suite
cargo bench --bench lexer_benchmarks
cargo bench --bench parser_benchmarks
cargo bench --bench semantic_benchmarks
cargo bench --bench vm_benchmarks
cargo bench --bench codegen_benchmarks
cargo bench --bench e2e_benchmarks

# Quick test run (faster)
cargo bench -- --test

# Run with specific parameters
cargo bench -- --sample-size 50 --measurement-time 3
```

### Viewing Results

Benchmark results are saved to `target/criterion/` with detailed Criterion reports:

- **Interactive HTML reports** - Open `target/criterion/report/index.html`
- **Individual benchmark data** - Detailed statistics and graphs for each test
- **Automatic baseline comparison** - Track performance changes over time
- **Statistical analysis** - Built-in confidence intervals and outlier detection

## Benchmark Details

### 1. Lexer Benchmarks (`lexer_benchmarks.rs`)

Tests tokenization performance across different scenarios:

- **Token scaling** - Performance vs. number of tokens (100 to 10,000 tokens)
- **Error handling** - Lexical error recovery and reporting performance

### 2. Parser Benchmarks (`parser_benchmarks.rs`)

Measures AST generation performance:

- **Expression parsing** - Simple to complex expressions
- **Function definitions** - Parsing function declarations
- **Scalability** - Performance with increasing input complexity
- **Error recovery** - Parser error handling performance

### 3. Semantic Benchmarks (`semantic_benchmarks.rs`)

Evaluates semantic analysis performance:

- **Type checking** - Type inference and validation
- **Scope resolution** - Variable and function scope handling
- **Function complexity** - Performance with many functions
- **Error analysis** - Semantic error detection performance

### 4. VM Benchmarks (`vm_benchmarks.rs`)

Tests bytecode execution performance:

- **Arithmetic operations** - Basic mathematical operations
- **Function calls** - Function call overhead and recursion
- **Memory operations** - Variable allocation and management
- **Scalability** - Performance with increasing operation counts

### 5. Code Generation Benchmarks (`codegen_benchmarks.rs`)

Measures compilation to bytecode performance:

- **Function generation** - Compiling function definitions
- **Expression compilation** - Complex expression code generation  
- **Nested scopes** - Handling nested block structures
- **Scalability** - Performance with large programs

### 6. End-to-End Benchmarks (`e2e_benchmarks.rs`)

Full compilation pipeline performance:

- **Integration testing** - Complete compilation process
- **Pipeline stages** - Individual stage performance measurement
- **Real-world programs** - Performance on typical code patterns
- **Scalability testing** - Large program compilation performance

## Test Program Generation

The `program_builder.rs` module provides utilities for generating test programs with controlled characteristics:

### Available Templates

- **`simple_arithmetic()`** - Basic arithmetic operations
- **`function_heavy(n)`** - Programs with many function definitions
- **`variable_heavy(n)`** - Programs with many variable declarations  
- **`deeply_nested(depth)`** - Nested block structures
- **`complex_expressions()`** - Complex mathematical expressions

### Generated Program Metadata

Each program includes complexity information:

- Variable count and types
- Function definitions and calls
- Expression complexity and nesting depth
- Control flow statements

## Implementation Details

### Direct Pipeline Usage

All benchmarks use the main compilation APIs directly:

```rust
// Direct compilation pipeline usage
use slang::compilation_pipeline::{self, CompilationPipeline, CompilationResult, PipelineStage};

// For full compilation
match compilation_pipeline::compile_to_bytecode(program, Some("benchmark.sl".to_string()), false) {
    CompilationResult::Success { chunk, .. } => Ok(chunk),
    CompilationResult::Failed { diagnostics } => Err(format!("Failed: {}", diagnostics.error_count()))
}

// For individual stages  
let pipeline = CompilationPipeline::new(program, Some("benchmark.sl".to_string()));
match pipeline.tokenize().and_then(|pipeline, tokens| pipeline.parse(tokens)) {
    PipelineStage::Success { data, .. } => Ok(data),
    PipelineStage::Failed { .. } => Err("Parse failed".to_string())
}
```

### Criterion Configuration

Benchmarks use standard Criterion patterns:

```rust
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};

let mut group = c.benchmark_group("Benchmark Name");
group.sample_size(100);
group.measurement_time(Duration::from_secs(5));

// Throughput benchmarks
group.throughput(Throughput::Elements(size as u64));
group.bench_with_input(BenchmarkId::new("test", size), &size, |b, &size| {
    let program = generate_program(size);
    b.iter(|| {
        compile_program(&program).expect("Should succeed")
    });
});
```

## Development Workflow

### Continuous Integration

The benchmarking suite integrates well with CI/CD:

- **Baseline comparison** - Criterion automatically compares against previous runs
- **Regression detection** - Performance changes are clearly visible in reports
- **Automated reporting** - HTML reports can be published as CI artifacts
- **Historical tracking** - Criterion maintains performance history

### Local Development

Use benchmarks during development:

- **Quick testing** - Use `cargo bench -- --test` for fast feedback
- **Targeted benchmarking** - Run specific benchmarks for areas being modified
- **Performance validation** - Verify optimization effectiveness
- **Regression prevention** - Catch performance issues early

## Best Practices

### Writing Benchmarks

- **Use direct APIs** - Call `CompilationPipeline` and `compilation_pipeline` directly
- **Include error cases** - Test error handling performance where relevant
- **Isolate components** - Test individual compiler stages separately  
- **Document purpose** - Clear comments about what each benchmark measures
- **Use appropriate sizes** - Match test complexity to what you're measuring

### Performance Analysis

- **Establish baselines** - Run benchmarks consistently to build baseline data
- **Monitor trends** - Look for gradual performance changes over time
- **Consider variance** - Account for measurement noise and system conditions
- **Use profiling** - Leverage external profilers for detailed analysis when needed

### Maintenance

- **Keep tests relevant** - Update test programs to reflect real-world usage
- **Review regularly** - Remove obsolete benchmarks and add new ones as needed
- **Update documentation** - Keep benchmark descriptions current
- **Validate assumptions** - Ensure benchmarks still test what they claim to test

## Troubleshooting

### Common Issues

- **High variance** - System load affecting measurements (close other applications)
- **Compilation failures** - Invalid generated test programs (check language syntax)
- **Performance regression false positives** - System configuration changes
- **Missing baseline data** - First run after clean checkout

### Performance Tips

- **Use release mode** - Always benchmark with `cargo bench` (release mode)
- **Consistent environment** - Use same hardware/OS configuration for comparisons
- **Reduce system noise** - Close unnecessary applications during benchmarking
- **Multiple runs** - Run benchmarks multiple times for statistical confidence

## Contributing

When adding new benchmarks:

1. **Follow patterns** - Use existing benchmarks as templates
2. **Include documentation** - Document what the benchmark measures and why
3. **Add appropriate test cases** - Include relevant scale and error testing
4. **Update this README** - Document new benchmark capabilities
5. **Test thoroughly** - Ensure benchmarks run reliably and measure correctly

## Architecture Benefits

The current simplified architecture provides:

- **Clarity** - Direct usage of main APIs without abstraction layers
- **Maintainability** - Fewer components to maintain and update
- **Reliability** - Proven Criterion.rs framework for all analysis
- **Performance** - No overhead from custom measurement infrastructure
- **Extensibility** - Easy to add new benchmarks following established patterns

The benchmarking suite focuses on providing reliable, actionable performance data while maintaining simplicity and ease of use.
