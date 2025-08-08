#![allow(dead_code)] // Benchmark programs may not all be actively used

use super::BenchmarkProgram;

/// Core benchmark programs - simple and commonly used
pub const SIMPLE_ARITHMETIC: BenchmarkProgram = BenchmarkProgram::new(
    "simple_arithmetic",
    r#"
let a = 10;
let b = 20;
let result = a + b * 2;
"#,
);

pub const SIMPLE_EXPRESSION: BenchmarkProgram =
    BenchmarkProgram::new("simple_expression", "let x = 1 + 2 * 3;");

pub const NESTED_EXPRESSIONS: BenchmarkProgram = BenchmarkProgram::new(
    "nested_expressions",
    "let result = ((a + b) * c) / (d - e);",
);

/// More complex arithmetic operations with multiple variables and operations
pub const COMPLEX_ARITHMETIC: BenchmarkProgram = BenchmarkProgram::new(
    "complex_arithmetic",
    r#"let a: i32 = 10;
let b: i32 = 20;
let c: i32 = 30;
let result = a + b * c - (a / b);
"#,
);

/// Complex mathematical expressions with multiple sub-expressions
pub const COMPLEX_EXPRESSIONS: BenchmarkProgram = BenchmarkProgram::new(
    "complex_expressions",
    r#"let a: i32 = 10;
let b: i32 = 20;
let c: i32 = 30;
let d: i32 = 40;
let expr1 = (a + b) * (c - d);
let expr2 = a * b + c * d;
let expr3 = (a + b) * (c + d) - (a - b) * (c - d);
let result = expr1 + expr2 + expr3;
"#,
);
