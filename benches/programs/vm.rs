#![allow(dead_code)] // Benchmark programs may not all be actively used

use super::BenchmarkProgram;

/// VM execution programs
pub const VM_SIMPLE_ARITHMETIC: BenchmarkProgram = BenchmarkProgram::new(
    "vm_simple_arithmetic",
    r#"
let a = 10;
let b = 20;
let result = a + b * 2 - 5;
print_value(result);
"#,
);

pub const VM_FUNCTION_CALLS: BenchmarkProgram = BenchmarkProgram::new(
    "vm_function_calls",
    r#"
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y;
}

let result = add(multiply(5, 3), add(2, 3));
print_value(result);
"#,
);

pub const VM_RECURSIVE_FIBONACCI: BenchmarkProgram = BenchmarkProgram::new(
    "vm_recursive_fibonacci",
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

pub const VM_NESTED_SCOPES: BenchmarkProgram = BenchmarkProgram::new(
    "vm_nested_scopes",
    r#"
let global = 1;
{
    let local = 2;
    {
        let inner = 3;
        let result = global + local + inner;
        print_value(result);
    }
}
"#,
);

/// VM value operation programs for benchmarking different data types and operations
pub const VM_INTEGER_ARITHMETIC: BenchmarkProgram = BenchmarkProgram::new(
    "integer_arithmetic",
    r#"
let a = 1000;
let b = 2000;
let result = a + b * a - b / a;
print_value(result);
"#,
);

pub const VM_FLOATING_POINT: BenchmarkProgram = BenchmarkProgram::new(
    "floating_point",
    r#"
let a = 3.14159;
let b = 2.71828;
let result = a * b + a / b;
print_value(result);
"#,
);

pub const VM_BOOLEAN_LOGIC: BenchmarkProgram = BenchmarkProgram::new(
    "boolean_logic",
    r#"
let a = true;
let b = false;
let result = (a && b) || (!a && !b) || (a != b);
print_value(result);
"#,
);

pub const VM_STRING_OPERATIONS: BenchmarkProgram = BenchmarkProgram::new(
    "string_operations",
    r#"
let str1 = "Hello";
let str2 = "World";
let result = str1 + " " + str2 + "!";
print_value(result);
"#,
);

pub const VM_COMPARISON_OPERATIONS: BenchmarkProgram = BenchmarkProgram::new(
    "comparison_operations",
    r#"
let x = 42;
let y = 24;
x > y;
x < y;
x == y;
x != y;
x >= y;
x <= y;
"#,
);
