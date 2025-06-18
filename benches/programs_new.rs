/// Program definitions for benchmarking - organized by category
/// This module re-exports all the test programs used across different benchmarks
/// 
/// Programs are now organized in separate modules by category:
/// - core: Basic arithmetic and expression programs
/// - functions: Function-related programs
/// - scopes: Scope and variable resolution programs  
/// - types: Type system and type checking programs
/// - e2e: End-to-end integration programs
/// - vm: Virtual machine execution programs
/// - errors: Error case programs for testing error handling

mod programs;

// Re-export everything from the programs module
pub use programs::*;
