mod programs;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use slang::compilation_pipeline::{CompilationPipeline, PipelineStage};
use slang_ir::ast::Statement;
use std::time::Duration;
use programs::{PARSER_PROGRAMS, PARSER_ERROR_PROGRAMS};

/// Helper function to parse only using CompilationPipeline
fn parse_only(program: &str) -> Result<Vec<Statement>, String> {
    let pipeline = CompilationPipeline::new(program, Some("benchmark.sl".to_string()), false);

    match pipeline
        .tokenize()
        .and_then(|pipeline, tokens| pipeline.parse(tokens))
    {
        PipelineStage::Success { data, .. } => Ok(data),
        PipelineStage::Failed { .. } => Err("AST compilation failed".to_string()),
    }
}

/// Benchmark parser performance with different AST complexities
fn parser_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parser Performance");

    group.sample_size(100);
    group.measurement_time(Duration::from_secs(5));

    for program in PARSER_PROGRAMS.iter() {
        group.bench_with_input(BenchmarkId::new("parse", program.name), &program.source, |b, program_source| {
            b.iter(|| parse_only(program_source).expect("Parse should succeed"));
        });
    }

    group.finish();
}

/// Benchmark parser scalability with increasing input sizes
fn parser_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parser Scalability");
    group.sample_size(50);

    let sizes = [10, 25, 50, 100, 200];

    for size in sizes.iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("nested_expressions", size),
            size,
            |b, &size| {
                let program = generate_nested_expressions(size);
                b.iter(|| parse_only(&program).expect("Parse should succeed"));
            },
        );
    }

    group.finish();
}

/// Generate nested expressions for scalability testing
fn generate_nested_expressions(depth: usize) -> String {
    let mut expr = "x".to_string();
    for i in 0..depth {
        expr = format!("({} + {})", expr, i);
    }
    format!("let result = {};", expr)
}

/// Benchmark parser error handling and recovery
fn parser_error_recovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parser Error Recovery");

    for program in PARSER_ERROR_PROGRAMS.iter() {
        group.bench_with_input(
            BenchmarkId::new("error_recovery", program.name),
            &program.source,
            |b, program_source| {
                b.iter(|| {
                    // Expect parsing to fail
                    let _ = parse_only(program_source);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    parser_performance,
    parser_scalability,
    parser_error_recovery
);
criterion_main!(benches);
