// Parser module public API
// Re-exports and coordinates all parser modules

// Module declarations
mod core;
mod error;
mod expressions;
mod literals;
mod statements;
mod types;
mod utilities;

pub use core::Parser;