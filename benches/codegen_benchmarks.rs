mod programs;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use slang::compilation_pipeline::{CompilationPipeline, CompilationResult};
use slang_backend::bytecode::Chunk;
use std::time::Duration;
use programs::templates::ProgramTemplates;
use programs::CODEGEN_PROGRAMS;

const CODEGEN_COMPLEXITY_LEVELS: [usize; 5] = [10, 20, 50, 100, 200];
const CODEGEN_FUNCTION_COUNTS: [usize; 3] = [10, 25, 50];
const CODEGEN_SCOPE_DEPTHS: [usize; 5] = [5, 10, 15, 20, 30];

/// Helper function to compile to bytecode using compilation_pipeline
fn compile_to_bytecode(program: &str) -> Result<Chunk, String> {
    let pipeline = CompilationPipeline::new(program, Some("benchmark.sl".to_string()), false);
    match pipeline.execute_all_stages() {
        CompilationResult::Success { chunk, .. } => Ok(chunk),
        CompilationResult::Failed { diagnostics } => {
            let error_msg = format!(
                "Compilation failed with {} errors",
                diagnostics.error_count()
            );
            Err(error_msg)
        }
    }
}

/// Benchmark code generation performance
fn codegen_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("Code Generation Performance");
    group.sample_size(100);
    group.measurement_time(Duration::from_secs(5));

    for program in CODEGEN_PROGRAMS.iter() {
        group.bench_with_input(BenchmarkId::new("codegen", program.name), &program.source, |b, program_source| {
            b.iter(|| compile_to_bytecode(program_source).expect("Code generation should succeed"));
        });
    }

    group.finish();
}

/// Benchmark code generation scalability
fn codegen_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("Code Generation Scalability");
    group.sample_size(50);

    // Test with increasing function complexity
    for complexity in CODEGEN_COMPLEXITY_LEVELS.iter() {
        group.throughput(Throughput::Elements(*complexity as u64));
        group.bench_with_input(
            BenchmarkId::new("complexity", complexity),
            complexity,
            |b, &complexity| {
                let program = ProgramTemplates::function_heavy(complexity);
                b.iter(|| {
                    compile_to_bytecode(&program.source).expect("Code generation should succeed")
                });
            },
        );
    }

    // Test with many functions
    for count in CODEGEN_FUNCTION_COUNTS.iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(
            BenchmarkId::new("many_functions", count),
            count,
            |b, &count| {
                let program = ProgramTemplates::function_heavy(count);
                b.iter(|| {
                    compile_to_bytecode(&program.source).expect("Code generation should succeed")
                });
            },
        );
    }

    for depth in CODEGEN_SCOPE_DEPTHS.iter() {
        group.throughput(Throughput::Elements(*depth as u64));
        group.bench_with_input(
            BenchmarkId::new("nested_scopes", depth),
            depth,
            |b, &depth| {
                let program = ProgramTemplates::deeply_nested(depth);
                b.iter(|| {
                    compile_to_bytecode(&program.source).expect("Code generation should succeed")
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, codegen_performance, codegen_scalability);
criterion_main!(benches);
