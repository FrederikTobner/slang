use slang_shared::{CompilationContext, SymbolKind};
use slang_types::TypeId;

/// Registers the built-in native functions that are available to all programs.
pub fn register_native_functions(context: &mut CompilationContext) {
    // Register print_value function
    // It accepts any type (TypeId::unknown() can represent this for now)
    // and returns an i32 (e.g., status code, though not strictly enforced here).
    let param_types = vec![TypeId::unknown()];
    let return_type = TypeId::i32();

    let function_type_id = context.register_function_type(param_types, return_type);

    // Define the 'print_value' symbol as a function.
    // The 'false' indicates it's not mutable, which is typical for functions.
    if context
        .define_symbol(
            "print_value".to_string(),
            SymbolKind::Function,
            function_type_id,
            false,
        )
        .is_err()
    {
        // This would ideally log an error or panic if registration fails,
        // as it indicates a fundamental issue with the compilation context setup.
        // For now, we'll let it pass, but in a production compiler, this should be handled.
        eprintln!("Error: Failed to register native function 'print_value'.");
    }

    // Add other native functions here in the future
    // Example:
    // let len_param_types = vec![TypeId::string()]; // Assuming a string type ID
    // let len_return_type = TypeId::i64(); // Or u64, i32 etc.
    // let len_function_type_id = context.register_function_type(len_param_types, len_return_type);
    // if context.define_symbol(
    //     "len".to_string(),
    //     SymbolKind::Function,
    //     len_function_type_id,
    //     false,
    // ).is_err() {
    //     eprintln!("Error: Failed to register native function 'len'.");
    // }
}
