pub mod compilation_context;
pub mod symbol_table;

// External imports
pub use compilation_context::CompilationContext;
pub use symbol_table::{SymbolTable, SymbolKind};

// Internal imports
use symbol_table::Symbol;