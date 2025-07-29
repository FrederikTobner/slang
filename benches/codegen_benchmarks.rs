mod programs;
mod utils;

use divan::{AllocProfiler, Bencher, black_box};
use programs::core::{COMPLEX_ARITHMETIC, SIMPLE_ARITHMETIC};
use programs::templates::ProgramTemplates;
use utils::pipeline::compile_to_bytecode;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

/// Benchmark code generation performance
#[divan::bench]
fn codegen_performance_simple(bencher: Bencher) {
    let program = &SIMPLE_ARITHMETIC;

    bencher.bench_local(|| {
        black_box(compile_to_bytecode(program.source).expect("Code generation should succeed"))
    });
}

#[divan::bench]
fn codegen_performance_complex(bencher: Bencher) {
    let program = &COMPLEX_ARITHMETIC;

    bencher.bench_local(|| {
        black_box(compile_to_bytecode(program.source).expect("Code generation should succeed"))
    });
}

/// Benchmark code generation scalability with complexity
#[divan::bench(args = [10, 20, 50, 100, 200])]
fn codegen_scalability_complexity(bencher: Bencher, complexity: usize) {
    let program = ProgramTemplates::function_heavy(complexity);

    bencher.bench_local(|| {
        black_box(compile_to_bytecode(&program.source).expect("Code generation should succeed"))
    });
}

/// Benchmark code generation with many functions
#[divan::bench(args = [10, 25, 50])]
fn codegen_scalability_many_functions(bencher: Bencher, count: usize) {
    let program = ProgramTemplates::function_heavy(count);

    bencher.bench_local(|| {
        black_box(compile_to_bytecode(&program.source).expect("Code generation should succeed"))
    });
}

/// Benchmark code generation with nested scopes
#[divan::bench(args = [5, 10, 15, 20, 30])]
fn codegen_scalability_nested_scopes(bencher: Bencher, depth: usize) {
    let program = ProgramTemplates::deeply_nested(depth);

    bencher.bench_local(|| {
        black_box(compile_to_bytecode(&program.source).expect("Code generation should succeed"))
    });
}

fn main() {
    divan::main();
}
