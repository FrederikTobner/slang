use super::BenchmarkProgram;

/// Type system programs
pub const SIMPLE_TYPES: BenchmarkProgram = BenchmarkProgram::new(
    "simple_types",
    r#"
let x: i32 = 42;
let y: f64 = 3.14;
let z: string = "hello";
let result = x + 10;
"#,
);

pub const TYPE_CHECKING: BenchmarkProgram = BenchmarkProgram::new(
    "type_checking",
    r#"
let x: i32 = 42;
let y: i64 = 100;
let z: f32 = 3.14;
let a: bool = true;
let b: string = "hello";

let int_result = x + 10;
let float_result = z * 2.0;
let bool_result = a && false;
let string_result = b + " world";
"#,
);

/// Array containing type programs for semantic analysis testing
pub const TYPE_SEMANTIC_PROGRAMS: &[&BenchmarkProgram] = &[&SIMPLE_TYPES, &TYPE_CHECKING];
