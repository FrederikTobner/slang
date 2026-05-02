use super::super::traits::ScopeManager;
use slang_shared::{CompilationContext, SymbolKind};
use slang_types::TypeId;

/// Context-based scope manager that implements scope lifecycle operations
/// using the compilation context's scoping mechanisms.
///
/// This implementation provides the standard scope management behavior
/// by delegating to the compilation context's scope operations.
pub struct ScopeStack<'a> {
    context: &'a mut CompilationContext,
}

impl<'a> ScopeStack<'a> {
    /// Create a new context-based scope manager
    ///
    /// # Arguments
    /// * `context` - The compilation context to manage scopes for
    pub fn new(context: &'a mut CompilationContext) -> Self {
        Self { context }
    }
}

impl ScopeManager for ScopeStack<'_> {
    fn enter_scope(&mut self) {
        self.context.begin_scope();
    }

    fn exit_scope(&mut self) -> Result<(), String> {
        self.context.end_scope()
    }

    fn define_symbol(
        &mut self,
        name: String,
        kind: SymbolKind,
        type_id: TypeId,
        is_mutable: bool,
    ) -> Result<(), String> {
        self.context.define_symbol(name, kind, type_id, is_mutable)
    }
}
