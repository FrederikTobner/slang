mod programs;
mod utils;

use divan::{Bencher, black_box, AllocProfiler};
use programs::core::{SIMPLE_EXPRESSION, NESTED_EXPRESSIONS};
use programs::errors::{ERROR_MISSING_SEMICOLON, ERROR_UNMATCHED_PAREN};
use utils::pipeline::parse_only;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

#[divan::bench]
fn parser_performance_simple(bencher: Bencher) {
    let program = &SIMPLE_EXPRESSION;
    
    bencher.bench_local(|| {
        
        
        black_box(parse_only(program.source).expect("Parse should succeed"))
    });
}

#[divan::bench]
fn parser_performance_complex(bencher: Bencher) {
    let program = &NESTED_EXPRESSIONS;
    
    bencher.bench_local(|| {
        
        
        black_box(parse_only(program.source).expect("Parse should succeed"))
    });
}

#[divan::bench(args = [10, 25, 50, 100, 200])]
fn parser_scalability_nested_expressions(bencher: Bencher, depth: usize) {
    let program = generate_nested_expressions(depth);
    
    bencher.bench_local(|| {
        
        
        black_box(parse_only(&program).expect("Parse should succeed"))
    });
}

fn generate_nested_expressions(depth: usize) -> String {
    let mut expr = "x".to_string();
    for i in 0..depth {
        expr = format!("({expr} + {i})");
    }
    format!("let result = {expr};")
}

#[divan::bench]
fn parser_error_recovery_0(bencher: Bencher) {
    let program = &ERROR_MISSING_SEMICOLON;
    
    bencher.bench_local(|| {
        
        // Expect parsing to fail
        
        
        black_box(parse_only(program.source))
    });
}

#[divan::bench]
fn parser_error_recovery_1(bencher: Bencher) {
    let program = &ERROR_UNMATCHED_PAREN;
    
    bencher.bench_local(|| {
        // Expect parsing to fail
        
        
        black_box(parse_only(program.source))
    });
}

fn main() {
    divan::main();
}
