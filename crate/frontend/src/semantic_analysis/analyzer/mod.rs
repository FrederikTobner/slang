pub mod core;
pub mod symbol_resolver;
pub mod scope_manager;
pub mod native_functions;

// Re-export the main analyzer for backward compatibility
pub use core::CoreAnalyzer;
