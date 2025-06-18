mod programs;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;

use programs::templates::ProgramTemplates;
use programs::LEXER_ERROR_PROGRAMS;

/// Benchmark the lexer performance with different input sizes
fn lexer_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lexer Performance");

    group.sample_size(100);
    group.measurement_time(Duration::from_secs(5));
    
    let sizes = [100, 500, 1000, 5000, 10000];
    
    for size in sizes.iter() {
        let program = ProgramTemplates::variable_heavy(*size);
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("tokenize", size),
            size,
            |b, _| {
                b.iter(|| {
                    let _ = slang_frontend::lexer::tokenize(&program.source);
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark lexer error handling performance
fn lexer_error_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lexer Error Handling");
    
    // Use relaxed statistical requirements for error handling
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(3));
    
    for program in LEXER_ERROR_PROGRAMS.iter() {
        group.bench_function(program.name, |b| {
            b.iter(|| {
                // we expect this to return an error
                let _ = slang_frontend::lexer::tokenize(program.source);
            })
        });
    }
    
    group.finish();
}

criterion_group!(benches, lexer_performance, lexer_error_handling);
criterion_main!(benches);
