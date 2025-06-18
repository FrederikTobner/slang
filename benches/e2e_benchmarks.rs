use criterion::{Criterion, criterion_group, criterion_main};
use slang::compilation_pipeline::CompilationPipeline;
use std::time::Duration;

mod programs;

use programs::E2E_PROGRAMS;
use programs::templates::ProgramTemplates;

const E2E_COMPLEXITY_LEVELS: [usize; 5] = [10, 20, 50, 100, 200];
const FIBONACCI_VALUES: [usize; 5] = [5, 10, 15, 20, 25];
const MEDIUM_TO_LARGE: [usize; 5] = [10, 50, 100, 200, 500];

/// Benchmark end-to-end compilation and execution performance
fn e2e_integration_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("End-to-End Integration Performance");

    group
        .sample_size(50)
        .measurement_time(Duration::new(5, 0))
        .bench_function("end_to_end_compilation", |b| {
            b.iter(|| {
                for program in E2E_PROGRAMS.iter() {
                    let _ =
                        CompilationPipeline::new(program.source, Some(program.name.to_string()), false)
                            .execute_all_stages();
                }
            });
        });
}

/// Benchmark end-to-end scalability
fn e2e_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("End-to-End Scalability");

    let mut programs = E2E_COMPLEXITY_LEVELS
        .iter()
        .map(|&complexity| ProgramTemplates::function_heavy(complexity))
        .collect::<Vec<_>>();
    for &complexity in &E2E_COMPLEXITY_LEVELS {
        programs.push(ProgramTemplates::function_heavy(complexity));
    }

    group
        .sample_size(50)
        .measurement_time(Duration::new(10, 0))
        .bench_function("e2e_program_complexity", |b| {
            b.iter(|| {
                for program in &programs {
                    let compilation_pipeline =
                        CompilationPipeline::new(&program.source, Some("test.sl".to_string()), false);
                    let _ = compilation_pipeline.execute_all_stages();
                }
            });
        });

    group.bench_function("e2e_fibonacci_recursive", |b| {
        b.iter(|| {
            for &n in &FIBONACCI_VALUES {
                let program = ProgramTemplates::function_heavy(n);
                let _ = CompilationPipeline::new(&program.source, Some("test.sl".to_string()), false)
                    .execute_all_stages();
            }
        });
    });
    let mut programs = vec![];
    for count in MEDIUM_TO_LARGE {
        programs.push(ProgramTemplates::variable_heavy(count));
    }
    group.bench_function("e2e_many_variables", |b| {
        b.iter(|| {
            for program in &programs {
                let compilation_pipeline =
                    CompilationPipeline::new(&program.source, Some("test.sl".to_string()), false);
                let _ = compilation_pipeline.execute_all_stages();
            }
        });
    });
}

/// Benchmark pipeline stages individually
fn pipeline_stages(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pipeline Stages");

    let test_program = ProgramTemplates::function_heavy(100);
    group.sample_size(50).bench_function("pipeline_stages", |b| {
        b.iter(|| {
            let compilation_pipeline =
                CompilationPipeline::new(&test_program.source, Some("test.sl".to_string()), false);
            let _ = compilation_pipeline.execute_all_stages();
        });
    });
}

criterion_group!(
    benches,
    e2e_integration_performance,
    e2e_scalability,
    pipeline_stages
);
criterion_main!(benches);
