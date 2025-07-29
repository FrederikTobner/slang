mod programs;
mod utils;

use divan::{Bencher, black_box};
use programs::errors::{ERROR_INVALID_CHAR, ERROR_UNTERMINATED_STRING, ERROR_INVALID_NUMBER};
use programs::templates::ProgramTemplates;
use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

#[divan::bench(args = [100, 500, 1000, 5000, 10000])]
fn lexer_performance(bencher: Bencher, size: usize) {
    let program = ProgramTemplates::variable_heavy(size);
    
        bencher.bench_local(|| {
        
        let lexer = slang_frontend::Lexer::new(&program.source);
        
        
        black_box(lexer.tokenize())
    });
}

#[divan::bench]  
fn lexer_error_handling_0(bencher: Bencher) {
    let program = &ERROR_INVALID_CHAR;
    
    bencher.bench_local(|| {
        
        let lexer = slang_frontend::Lexer::new(program.source);
        
        
        black_box(lexer.tokenize())
    });
}

#[divan::bench]
fn lexer_error_handling_1(bencher: Bencher) {
    let program = &ERROR_UNTERMINATED_STRING;
    
    bencher.bench_local(|| {
        let lexer = slang_frontend::Lexer::new(program.source);
        
        
        black_box(lexer.tokenize())
    });
}

#[divan::bench]
fn lexer_error_handling_2(bencher: Bencher) {
    let program = &ERROR_INVALID_NUMBER;
    
    bencher.bench_local(|| {
        let lexer = slang_frontend::Lexer::new(program.source);
        black_box(lexer.tokenize())
    });
}

fn main() {
    divan::main();
}
