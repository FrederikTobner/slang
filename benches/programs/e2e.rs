use super::BenchmarkProgram;

/// End-to-end programs with print statements
pub const E2E_SIMPLE_ARITHMETIC: BenchmarkProgram = BenchmarkProgram::new(
    "e2e_simple_arithmetic",
    r#"
let a = 10;
let b = 20;
let result = a + b * 2;
print_value(result);
"#,
);

pub const E2E_FIBONACCI_RECURSIVE: BenchmarkProgram = BenchmarkProgram::new(
    "e2e_fibonacci_recursive",
    r#"
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

let result = fibonacci(10);
print_value(result);
"#,
);

pub const E2E_NESTED_SCOPES: BenchmarkProgram = BenchmarkProgram::new(
    "e2e_nested_scopes",
    r#"
let x = 1;
{
    let y = 2;
    {
        let z = 3;
        let result = x + y + z;
        print_value(result);
    }
}
"#,
);

pub const E2E_FUNCTION_DEFINITIONS: BenchmarkProgram = BenchmarkProgram::new(
    "e2e_function_definitions",
    r#"
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

fn calculate(x: i32, y: i32) -> i32 {
    let sum = add(x, y);
    let product = multiply(sum, 2);
    return product;
}

let result = calculate(5, 3);
print_value(result);
"#,
);

pub const E2E_CONTROL_FLOW: BenchmarkProgram = BenchmarkProgram::new(
    "e2e_control_flow",
    r#"
fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

let result = factorial(5);
print_value(result);
"#,
);
