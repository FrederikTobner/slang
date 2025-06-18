mod programs;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use slang_backend::VM;
use std::time::Duration;
use slang::compilation_pipeline::{CompilationPipeline, CompilationResult};
use programs::templates::ProgramTemplates;
use programs::{VM_PROGRAMS, VM_VALUE_OPERATION_PROGRAMS};

/// Helper function to execute a program using compilation_pipeline
fn execute_program(program: &str) -> Result<(), String> {
        let mut vm = VM::new();
        let pipeline = CompilationPipeline::new(program, Some("benchmark.sl".to_string()), false);
        match pipeline.execute_all_stages() {
        CompilationResult::Success { chunk, .. } => {
            match vm.interpret(&chunk) {
                Ok(()) => Ok(()),
                Err(err) => Err(format!("VM execution failed: {}", err)),
            }
        },
        CompilationResult::Failed { diagnostics } => {
            let error_msg = format!("Compilation failed with {} errors", diagnostics.error_count());
            Err(error_msg)
        }
    }
}

/// Benchmark virtual machine execution performance
fn vm_execution_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("VM Execution Performance");
    group.sample_size(100);
    group.measurement_time(Duration::from_secs(5));
    
    for program in VM_PROGRAMS.iter() {
        group.bench_with_input(BenchmarkId::new("vm_execute", program.name), &program.source, |b, program_source| {
            b.iter(|| {
                execute_program(program_source).expect("VM execution should succeed")
            });
        });
    }
    
    group.finish();
}

/// Benchmark VM scalability with increasing computational complexity
fn vm_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("VM Scalability");
    group.sample_size(50);
    
    // Test with increasing recursion depth (Fibonacci)
    let fibonacci_depths = [5, 10, 15, 20];
    for depth in fibonacci_depths.iter() {
        group.throughput(Throughput::Elements(*depth as u64));
        group.bench_with_input(BenchmarkId::new("fibonacci_depth", depth), depth, |b, &depth| {
            let program = format!(r#"
                fn fibonacci(n: i32) -> i32 {{
                    if n <= 1 {{
                        return n;
                    }}
                    return fibonacci(n - 1) + fibonacci(n - 2);
                }}
                let result = fibonacci({});
                print_value(result);
            "#, depth);
            b.iter(|| {
                execute_program(&program).expect("VM execution should succeed")
            });
        });
    }
    
    // Test with increasing function call complexity
    let function_counts = [5, 10, 25, 50];
    for count in function_counts.iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(BenchmarkId::new("function_calls", count), count, |b, &count| {
            let program = ProgramTemplates::function_heavy(count);
            b.iter(|| {
                execute_program(&program.source).expect("VM execution should succeed")
            });
        });
    }
    
    group.finish();
}

/// Benchmark VM value operations (arithmetic, comparisons, etc.)
fn vm_value_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("VM Value Operations");
    group.sample_size(100);
    
    for program in VM_VALUE_OPERATION_PROGRAMS.iter() {
        group.bench_with_input(BenchmarkId::new("value_operations", program.name), &program.source, |b, program_source| {
            b.iter(|| {
                execute_program(program_source).expect("VM execution should succeed")
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, vm_execution_performance, vm_scalability, vm_value_operations);
criterion_main!(benches);
