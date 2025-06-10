// Core traits for semantic analysis components
use slang_shared::{Symbol, SymbolKind};
use slang_types::TypeId;
use super::error::SemanticAnalysisError;

/// Type alias for result of semantic analysis operations
/// Contains either a valid TypeId or a SemanticAnalysisError
pub type SemanticResult = Result<TypeId, SemanticAnalysisError>;

/// Trait for symbol resolution operations
/// 
/// This trait abstracts symbol lookup operations, allowing for different
/// implementations and easier testing through mocking.
pub trait SymbolResolver {
    /// Resolve a variable symbol by name
    /// 
    /// Returns the symbol if found and it's a variable, None otherwise
    fn resolve_variable(&self, name: &str) -> Option<&Symbol>;
    
    /// Resolve a function symbol by name
    /// 
    /// Returns the symbol if found and it's a function, None otherwise
    fn resolve_function(&self, name: &str) -> Option<&Symbol>;
    
    /// Resolve a symbol that can be used as a value (variables and functions)
    /// 
    /// This allows functions to be accessed as first-class values
    fn resolve_value(&self, name: &str) -> Option<&Symbol>;
}

/// Trait for scope management operations
/// 
/// This trait abstracts scope lifecycle management, enabling different
/// scoping strategies and easier testing.
pub trait ScopeManager {
    /// Enter a new scope
    fn enter_scope(&mut self);
    
    /// Exit the current scope
    fn exit_scope(&mut self);
    
    /// Define a symbol in the current scope
    fn define_symbol(
        &mut self, 
        name: String, 
        kind: SymbolKind, 
        type_id: TypeId, 
        is_mutable: bool
    ) -> Result<(), String>;
}
