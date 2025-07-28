mod programs;
mod utils;

use divan::{Bencher, black_box, AllocProfiler};
use programs::e2e::{E2E_SIMPLE_ARITHMETIC, E2E_FIBONACCI_RECURSIVE, E2E_NESTED_SCOPES, E2E_FUNCTION_DEFINITIONS, E2E_CONTROL_FLOW};
use programs::templates::ProgramTemplates;
use slang::compilation_pipeline::CompilationPipeline;
use utils::pipeline::compile_to_bytecode;

const E2E_COMPLEXITY_LEVELS: [usize; 5] = [10, 20, 50, 100, 200];
const FIBONACCI_VALUES: [usize; 5] = [5, 10, 15, 20, 25];
const MEDIUM_TO_LARGE: [usize; 5] = [10, 50, 100, 200, 500];

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

#[divan::bench]
fn end_to_end_compilation(bencher: Bencher) {
    bencher.bench(|| {
        let programs = [&E2E_SIMPLE_ARITHMETIC, &E2E_FIBONACCI_RECURSIVE, &E2E_NESTED_SCOPES, &E2E_FUNCTION_DEFINITIONS, &E2E_CONTROL_FLOW];
        for program in programs.iter() {
            let _ = black_box(CompilationPipeline::new(
                program.source,
                Some(program.name.to_string()),
                false,
            )
            .execute_all_stages());
        }
    });
}

#[divan::bench(args = E2E_COMPLEXITY_LEVELS)]
fn e2e_program_complexity(bencher: Bencher, complexity: usize) {
    let program = ProgramTemplates::function_heavy(complexity);
    
    bencher.bench(|| {
        let compilation_pipeline = CompilationPipeline::new(
            &program.source,
            Some("test.sl".to_string()),
            false,
        );
        let _ = black_box(compilation_pipeline.execute_all_stages());
    });
}

#[divan::bench(args = FIBONACCI_VALUES)]
fn e2e_fibonacci_recursive(bencher: Bencher, n: usize) {
    
    let program = ProgramTemplates::function_heavy(n);
    
    bencher.bench(|| {
        let _ = black_box(
            CompilationPipeline::new(&program.source, Some("test.sl".to_string()), false)
                .execute_all_stages()
        );
    });
}

#[divan::bench(args = MEDIUM_TO_LARGE)]
fn e2e_many_variables(bencher: Bencher, count: usize) {
    
    let program = ProgramTemplates::variable_heavy(count);
    
    bencher.bench(|| {
        let compilation_pipeline =
            CompilationPipeline::new(&program.source, Some("test.sl".to_string()), false);
        let _ = black_box(compilation_pipeline.execute_all_stages());
    });
}

#[divan::bench]
fn pipeline_stages(bencher: Bencher) {
    
    let test_program = ProgramTemplates::function_heavy(100);
    
    bencher.bench(|| {
        black_box(
            compile_to_bytecode(&test_program.source)
                .expect("Pipeline stages should execute successfully")
        );
    });
}

fn main() {
    divan::main();
}
