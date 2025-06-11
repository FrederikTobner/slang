pub mod core;
pub mod native_functions;
pub mod scope_manager;
pub mod symbol_resolver; // Ensure this module is declared

// Re-export the main analyzer for backward compatibility
pub use core::CoreAnalyzer;
