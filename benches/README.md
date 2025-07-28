# Performance Testing Suite for Slang Compiler

This directory contains a streamlined performance testing suite for the Slang compiler using [Divan](https://github.com/nvzqz/divan) for comprehensive benchmarking and performance analysis.

## Architecture

### Benchmark Files

- **`lexer_benchmarks.rs`** - Tokenization performance across different input sizes and error cases
- **`parser_benchmarks.rs`** - AST generation and parsing performance with scalability testing  
- **`semantic_benchmarks.rs`** - Type checking and semantic analysis performance
- **`vm_benchmarks.rs`** - Bytecode execution and VM performance testing
- **`codegen_benchmarks.rs`** - Code generation and compilation performance
- **`e2e_benchmarks.rs`** - End-to-end compilation pipeline benchmarks

### Common Utilities (`utils/` directory)

- **`pipeline.rs`** - Compilation pipeline utilities for stage-specific testing
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
cargo bench -- --sample-count 10 --max-time 5

# Filter specific benchmarks
cargo bench -- lexer_performance
cargo bench -- --exact error_handling

# Get detailed memory analysis
cargo bench -- --color always
```

### Viewing Results

Divan provides immediate, comprehensive results in the terminal:

- **Real-time output** - Results displayed immediately after each benchmark
- **Memory profiling** - Built-in allocation tracking with detailed statistics
- **Statistical analysis** - Automatic confidence intervals and performance metrics
- **Compact format** - Clean tabular output with timing and memory data
- **Comparison support** - Easy performance regression detection

## Benchmark Details

### 1. Lexer Benchmarks (`lexer_benchmarks.rs`)

Tests tokenization performance with integrated memory tracking:

- **Token scaling** - Performance vs. number of tokens (100 to 10,000 tokens)
- **Error handling** - Lexical error recovery and reporting performance
- **Memory usage** - Allocation patterns during tokenization

### 2. Parser Benchmarks (`parser_benchmarks.rs`)

Measures AST generation performance with memory profiling:

- **Expression parsing** - Simple to complex expressions
- **Function definitions** - Parsing function declarations
- **Scalability** - Performance with increasing input complexity
- **Memory efficiency** - AST allocation patterns and memory usage

### 3. Semantic Benchmarks (`semantic_benchmarks.rs`)

Evaluates semantic analysis performance with allocation tracking:

- **Type checking** - Type inference and validation
- **Scope resolution** - Variable and function scope handling
- **Function complexity** - Performance with many functions
- **Memory patterns** - Symbol table and type information allocation

### 4. VM Benchmarks (`vm_benchmarks.rs`)

Tests bytecode execution performance with memory monitoring:

- **Arithmetic operations** - Basic mathematical operations
- **Function calls** - Function call overhead and recursion
- **Memory operations** - Variable allocation and management
- **Runtime efficiency** - Stack and heap usage during execution

### 5. Code Generation Benchmarks (`codegen_benchmarks.rs`)

Measures compilation to bytecode performance with allocation analysis:

- **Function generation** - Compiling function definitions
- **Expression compilation** - Complex expression code generation  
- **Nested scopes** - Handling nested block structures
- **Bytecode efficiency** - Code generation memory usage patterns

### 6. End-to-End Benchmarks (`e2e_benchmarks.rs`)

Full compilation pipeline performance with comprehensive memory tracking:

- **Integration testing** - Complete compilation process
- **Pipeline stages** - Individual stage performance measurement
- **Real-world programs** - Performance on typical code patterns
- **Total resource usage** - End-to-end timing and memory consumption

## Test Program Generation

The `programs/` module provides utilities for generating test programs with controlled characteristics:

### Available Test Programs

- **`programs/core.rs`** - Basic arithmetic and expression programs
- **`programs/errors.rs`** - Programs designed to trigger specific error conditions
- **`programs/functions.rs`** - Function definition and call patterns
- **`programs/templates.rs`** - Dynamic program generation with scaling complexity
- **`programs/types.rs`** - Type system testing programs
- **`programs/vm.rs`** - Virtual machine execution test programs

### Program Templates

The `ProgramTemplates` provides scalable test generation:

- **`variable_heavy(n)`** - Programs with many variable declarations  
- **`function_heavy(n)`** - Programs with many function definitions
- **`deeply_nested(depth)`** - Nested block structures

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

### Divan Configuration and Memory Profiling

Benchmarks use Divan's modern benchmarking patterns with integrated memory tracking:

```rust
use divan::{Bencher, black_box, AllocProfiler};

// Global allocator for memory profiling
#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

// Parameterized benchmarks
#[divan::bench(args = [100, 500, 1000, 5000, 10000])]
fn lexer_performance(bencher: Bencher, size: usize) {
    let program = ProgramTemplates::variable_heavy(size);
    
    bencher.bench_local(|| {
        let lexer = slang_frontend::Lexer::new(&program.source);
        black_box(lexer.tokenize())
    });
}

// Simple benchmarks
#[divan::bench]
fn parser_expression(bencher: Bencher) {
    let program = &COMPLEX_ARITHMETIC;
    
    bencher.bench_local(|| {
        black_box(parse_program(&program.source))
    });
}
```

### Memory Tracking Features

Divan's `AllocProfiler` provides comprehensive memory analysis:

- **`max alloc`** - Peak number of allocations and bytes during benchmark
- **`alloc`** - Total allocations during benchmark execution  
- **`grow`** - Memory growth operations and total bytes grown

## Development Workflow

### Real-time Performance Analysis

Divan provides immediate, actionable performance data:

- **Real-time feedback** - Results appear immediately in the terminal
- **Memory insights** - Built-in allocation profiling reveals memory bottlenecks
- **Statistical confidence** - Automatic statistical analysis of timing variance
- **Comparison support** - Easy before/after performance comparison
- **Fast execution** - Optimized measurement reduces benchmark runtime

### Local Development

Use benchmarks during development:

- **Quick feedback** - Use `cargo bench -- --test` for fast validation
- **Targeted testing** - Run specific benchmarks for areas being modified
- **Memory debugging** - Use allocation profiling to identify memory issues
- **Performance validation** - Verify optimization effectiveness immediately

## Best Practices

### Writing Benchmarks

- **Use direct APIs** - Call `CompilationPipeline` and `compilation_pipeline` directly
- **Leverage memory profiling** - Use `AllocProfiler` to understand allocation patterns
- **Include diverse test cases** - Test various input sizes and complexity levels
- **Document purpose** - Clear comments about what each benchmark measures
- **Use `black_box`** - Prevent compiler optimizations from skewing results

### Performance Analysis

- **Monitor both metrics** - Track timing and memory usage together
- **Look for patterns** - Identify scaling behavior and memory allocation trends
- **Consider variance** - Account for measurement noise and system conditions
- **Compare consistently** - Use same environment for performance comparisons

### Maintenance

- **Keep tests relevant** - Update test programs to reflect real-world usage
- **Review regularly** - Remove obsolete benchmarks and add new ones as needed
- **Update documentation** - Keep benchmark descriptions current
- **Validate measurements** - Ensure benchmarks still test what they claim to test

## Troubleshooting

### Common Issues

- **High variance** - System load affecting measurements (close other applications)
- **Compilation failures** - Invalid generated test programs (check language syntax)
- **Memory allocation inconsistencies** - Different system memory states
- **Missing allocator setup** - Ensure `AllocProfiler` is configured as global allocator

### Performance Tips

- **Use release mode** - Always benchmark with `cargo bench` (release mode)
- **Consistent environment** - Use same hardware/OS configuration for comparisons
- **Reduce system noise** - Close unnecessary applications during benchmarking
- **Monitor memory** - Watch for memory leaks or unexpected allocation patterns

## Contributing

When adding new benchmarks:

1. **Follow patterns** - Use existing benchmarks as templates
2. **Include memory profiling** - Set up `AllocProfiler` for allocation tracking
3. **Add appropriate test cases** - Include relevant scale and complexity testing
4. **Update this README** - Document new benchmark capabilities
5. **Test thoroughly** - Ensure benchmarks run reliably and measure correctly

## Architecture Benefits

The current Divan-based architecture provides:

- **Modern framework** - Latest benchmarking technology with active development
- **Integrated profiling** - Built-in memory allocation tracking without external tools
- **Fast execution** - Optimized measurement infrastructure for quick feedback
- **Clean output** - Readable terminal results with timing and memory data
- **Minimal overhead** - Efficient measurement with low performance impact
- **Extensibility** - Easy to add new benchmarks following established patterns

The benchmarking suite focuses on providing reliable, actionable performance data with integrated memory analysis while maintaining simplicity and fast execution.
