/// Program definitions for benchmarking
/// This module contains all the test programs used across different benchmarks
pub mod core;
pub mod e2e;
pub mod errors;
pub mod functions;
pub mod scopes;
pub mod templates;
pub mod types;
pub mod vm;

#[derive(Debug, Clone)]
pub struct BenchmarkProgram {
    #[allow(dead_code)] // Used in benchmarks which may not all be active
    pub name: &'static str,
    pub source: &'static str,
}

impl BenchmarkProgram {
    pub const fn new(name: &'static str, source: &'static str) -> Self {
        Self { name, source }
    }
}
