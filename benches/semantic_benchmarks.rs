mod programs;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;
use slang::compilation_pipeline::{CompilationPipeline, PipelineStage};
use slang_ir::ast::Statement;
use programs::templates::ProgramTemplates;
use programs::{SEMANTIC_PROGRAMS, SEMANTIC_ERROR_PROGRAMS};

/// Helper function to perform semantic analysis using CompilationPipeline
fn semantic_analysis_only(program: &str) -> Result<Vec<Statement>, String> {
    let pipeline = CompilationPipeline::new(program, Some("benchmark.sl".to_string()), false);
    
    match pipeline
        .tokenize()
        .and_then(|pipeline, tokens| pipeline.parse(tokens))
        .and_then(|pipeline, statements| pipeline.semantic_analysis(statements))
    {
        PipelineStage::Success { data, .. } => Ok(data),
        PipelineStage::Failed { .. } => {
            Err("Semantic analysis failed".to_string())
        }
    }
}

/// Benchmark semantic analysis performance
fn semantic_analysis_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("Semantic Analysis Performance");
    group.sample_size(100);
    group.measurement_time(Duration::from_secs(5));
    
    for program in SEMANTIC_PROGRAMS.iter() {
        group.bench_with_input(BenchmarkId::new("semantic_analysis", program.name), &program.source, |b, program_source| {
            b.iter(|| {
                semantic_analysis_only(program_source).expect("Semantic analysis should succeed")
            });
        });
    }
    
    group.finish();
}

/// Benchmark semantic analysis scalability
fn semantic_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("Semantic Analysis Scalability");
    group.sample_size(50);
    
    let variable_counts = [10, 25, 50, 100, 200];
    for count in variable_counts.iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(BenchmarkId::new("variable_declarations", count), count, |b, &count| {
            let program = generate_variable_declarations(count);
            b.iter(|| {
                semantic_analysis_only(&program).expect("Semantic analysis should succeed")
            });
        });
    }
    
    let scope_depths = [5, 10, 15, 20, 30];
    for depth in scope_depths.iter() {
        group.throughput(Throughput::Elements(*depth as u64));
        group.bench_with_input(BenchmarkId::new("scope_depth", depth), depth, |b, &depth| {
            let program = ProgramTemplates::deeply_nested(depth);
            b.iter(|| {
                semantic_analysis_only(&program.source).expect("Semantic analysis should succeed")
            });
        });
    }
    
    let function_complexities = [5, 10, 25, 50, 100];
    for complexity in function_complexities.iter() {
        group.throughput(Throughput::Elements(*complexity as u64));
        group.bench_with_input(BenchmarkId::new("function_complexity", complexity), complexity, |b, &complexity| {
            let program = ProgramTemplates::function_heavy(complexity);
            b.iter(|| {
                semantic_analysis_only(&program.source).expect("Semantic analysis should succeed")
            });
        });
    }
    
    group.finish();
}

/// Generate variable declarations for scalability testing
fn generate_variable_declarations(count: usize) -> String {
    let mut program = String::new();
    for i in 0..count {
        program.push_str(&format!("let var_{}: i32 = {};\n", i, i));
    }
    program
}

/// Benchmark semantic error handling
fn semantic_error_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Semantic Error Handling");
    group.sample_size(100);
    
    for program in SEMANTIC_ERROR_PROGRAMS.iter() {
        group.bench_with_input(BenchmarkId::new("semantic_error", program.name), &program.source, |b, program_source| {
            b.iter(|| {
                // Expect semantic analysis to fail but measure error handling time
                let _ = semantic_analysis_only(program_source);
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, semantic_analysis_performance, semantic_scalability, semantic_error_handling);
criterion_main!(benches);
