mod programs;
mod utils;

use divan::{Bencher, black_box, AllocProfiler};
use programs::templates::ProgramTemplates;
use programs::types::SIMPLE_TYPES;
use programs::functions::FUNCTION_CALLS;
use programs::errors::{ERROR_UNDEFINED_VARIABLE, ERROR_TYPE_MISMATCH};
use utils::pipeline::semantic_analysis_only;


#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

#[divan::bench]
fn semantic_analysis_performance_simple(bencher: Bencher) {
    let program = &SIMPLE_TYPES;
    
    bencher.bench_local(|| {
        black_box(semantic_analysis_only(program.source).expect("Semantic analysis should succeed"))
    });
}

#[divan::bench]
fn semantic_analysis_performance_complex(bencher: Bencher) {
    let program = &FUNCTION_CALLS;
    
    bencher.bench_local(|| {
        black_box(semantic_analysis_only(program.source).expect("Semantic analysis should succeed"))
    });
}

#[divan::bench(args = [10, 25, 50, 100, 200])]
fn semantic_scalability_variables(bencher: Bencher, count: usize) {
    let program = generate_variable_declarations(count);
    
    bencher.bench_local(|| {
        black_box(semantic_analysis_only(&program).expect("Semantic analysis should succeed"));
    });
}

#[divan::bench(args = [5, 10, 15, 20, 30])]
fn semantic_scalability_scope_depth(bencher: Bencher, depth: usize) {
    let program = ProgramTemplates::deeply_nested(depth);
    
    bencher.bench_local(|| {
        black_box(semantic_analysis_only(&program.source).expect("Semantic analysis should succeed"));
    });
}

#[divan::bench(args = [5, 10, 25, 50, 100])]
fn semantic_scalability_function_complexity(bencher: Bencher, complexity: usize) {
    let program = ProgramTemplates::function_heavy(complexity);
    
    bencher.bench_local(|| {
        black_box(semantic_analysis_only(&program.source).expect("Semantic analysis should succeed"));
    });
}

fn generate_variable_declarations(count: usize) -> String {
    let mut program = String::new();
    for i in 0..count {
        program.push_str(&format!("let var_{i}: i32 = {i};\n"));
    }
    program
}

#[divan::bench]
fn semantic_error_handling_0(bencher: Bencher) {
    let program = &ERROR_UNDEFINED_VARIABLE;
    
    bencher.bench_local(|| {
        black_box(semantic_analysis_only(program.source))
    });
}

#[divan::bench]
fn semantic_error_handling_1(bencher: Bencher) {
    let program = &ERROR_TYPE_MISMATCH;
    
    bencher.bench_local(|| {
         black_box(semantic_analysis_only(program.source))
    });
}

fn main() {
    divan::main();
}
