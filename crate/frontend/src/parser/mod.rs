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

// Re-export the main parse function
pub use self::core::parse;
