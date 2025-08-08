#![allow(dead_code)]  // Benchmark programs may not all be actively used

use super::BenchmarkProgram;

/// Function-related programs
pub const FUNCTION_CALLS: BenchmarkProgram = BenchmarkProgram::new(
    "function_calls",
    r#"
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y;
}

let result = add(multiply(5, 3), 7);
"#,
);

pub const FUNCTION_DEFINITION: BenchmarkProgram = BenchmarkProgram::new(
    "function_definition",
    r#"
fn calculate(x: i32, y: i32) -> i32 {
    return x * y + 42;
}
"#,
);

pub const COMPLEX_FUNCTION: BenchmarkProgram = BenchmarkProgram::new(
    "complex_function",
    r#"
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
"#,
);

pub const CONTROL_FLOW: BenchmarkProgram = BenchmarkProgram::new(
    "control_flow",
    r#"
fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

let result = factorial(5);
"#,
);

/// Array containing function programs for parser testing
pub const FUNCTION_PARSER_PROGRAMS: &[&BenchmarkProgram] =
    &[&FUNCTION_DEFINITION, &COMPLEX_FUNCTION];

/// Array containing function programs for codegen testing
pub const FUNCTION_CODEGEN_PROGRAMS: &[&BenchmarkProgram] = &[&FUNCTION_CALLS, &CONTROL_FLOW];
