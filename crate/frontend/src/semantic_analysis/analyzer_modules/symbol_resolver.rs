use slang_shared::{CompilationContext, Symbol, SymbolKind};
use super::super::traits::SymbolResolver;

/// Context-based symbol resolver that implements symbol lookup operations
/// using the compilation context's symbol table.
/// 
/// This implementation provides the standard symbol resolution behavior
/// by delegating to the compilation context's symbol lookup mechanisms.
pub struct ContextSymbolResolver<'a> {
    context: &'a CompilationContext,
}

impl<'a> ContextSymbolResolver<'a> {
    /// Create a new context-based symbol resolver
    /// 
    /// # Arguments
    /// * `context` - The compilation context containing the symbol table
    pub fn new(context: &'a CompilationContext) -> Self {
        Self { context }
    }
}

impl<'a> SymbolResolver for ContextSymbolResolver<'a> {
    fn resolve_variable(&self, name: &str) -> Option<&Symbol> {
        self.context.lookup_symbol(name)
            .filter(|symbol| symbol.kind() == SymbolKind::Variable)
    }

    fn resolve_function(&self, name: &str) -> Option<&Symbol> {
        self.context.lookup_symbol(name)
            .filter(|symbol| symbol.kind() == SymbolKind::Function)
    }

    fn resolve_value(&self, name: &str) -> Option<&Symbol> {
        self.context.lookup_symbol(name)
            .filter(|symbol| matches!(symbol.kind(), SymbolKind::Variable | SymbolKind::Function))
    }
}
