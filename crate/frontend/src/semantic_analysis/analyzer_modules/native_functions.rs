use slang_shared::{CompilationContext, SymbolKind};
use slang_types::{TypeId};

/// Register built-in native functions that are available to all programs.
/// 
/// This function sets up the built-in functions in the compilation context,
/// making them available for use in semantic analysis.
/// 
/// # Arguments
/// * `context` - The compilation context to register functions in
pub fn register_builtins(context: &mut CompilationContext) {
    register_print_value_function(context);
}

/// Register the print_value function that accepts any type and returns i32
/// 
/// # Arguments
/// * `context` - The compilation context to register the function in
fn register_print_value_function(context: &mut CompilationContext) {
    let param_types = vec![TypeId::unknown()];
    let return_type = TypeId::i32();

    // Register as a function symbol in the symbol table
    let function_type_id = context.register_function_type(param_types, return_type);
    let _ = context.define_symbol(
        "print_value".to_string(), 
        SymbolKind::Function, 
        function_type_id, 
        false
    );
}
