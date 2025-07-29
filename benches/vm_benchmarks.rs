mod programs;
mod utils;

use divan::{Bencher, black_box, AllocProfiler};
use programs::templates::ProgramTemplates;
use programs::vm::{VM_SIMPLE_ARITHMETIC, VM_FUNCTION_CALLS, VM_INTEGER_ARITHMETIC, VM_FLOATING_POINT};
use utils::pipeline::execute_program;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

#[divan::bench]
fn vm_execution_performance_simple(bencher: Bencher) {
    let program = &VM_SIMPLE_ARITHMETIC;
    
    bencher.bench_local(|| {
        execute_program(program.source).expect("VM execution should succeed");
        black_box(())
    });
}

#[divan::bench]
fn vm_execution_performance_complex(bencher: Bencher) {
    let program = &VM_FUNCTION_CALLS;
    
    bencher.bench_local(|| {
        execute_program(program.source).expect("VM execution should succeed");
        black_box(())
    });
}

#[divan::bench(args = [5, 10, 15, 20])]
fn vm_scalability_fibonacci_depth(bencher: Bencher, depth: usize) {
    let program = format!(
        r#"
        fn fibonacci(n: i32) -> i32 {{
            if n <= 1 {{
                return n;
            }}
            return fibonacci(n - 1) + fibonacci(n - 2);
        }}
        let result = fibonacci({depth});
        print_value(result);
    "#
    );
    
    bencher.bench_local(|| {
        execute_program(&program).expect("VM execution should succeed");
        black_box(())
    });
}

#[divan::bench(args = [5, 10, 25, 50])]
fn vm_scalability_function_calls(bencher: Bencher, count: usize) {
    let program = ProgramTemplates::function_heavy(count);
    
    bencher.bench_local(|| {
         execute_program(&program.source).expect("VM execution should succeed");
         black_box(())
    });
}

#[divan::bench]
fn vm_value_operations_0(bencher: Bencher) {
    let program = &VM_INTEGER_ARITHMETIC;
    
    bencher.bench_local(|| {
        execute_program(program.source).expect("VM execution should succeed");
        black_box(())
    });
}

#[divan::bench]
fn vm_value_operations_1(bencher: Bencher) {
    let program = &VM_FLOATING_POINT;
    
    bencher.bench_local(|| {
        execute_program(program.source).expect("VM execution should succeed");
        black_box(())
    });
}

fn main() {
    divan::main();
}
