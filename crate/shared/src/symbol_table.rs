use slang_types::types::TypeId;
use std::collections::HashMap;

/// Represents the kind of a symbol in the symbol table
/// 
/// This enum categorizes symbols by their role in the language:
/// - Types (built-in or user-defined)
/// - Variables (local or global)
/// - Functions (user-defined or built-in)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    /// A type symbol (primitive types, structs, enums, etc.)
    Type,
    /// A variable symbol (let declarations, function parameters, etc.)
    Variable,
    /// A function symbol (function declarations, built-in functions, etc.)
    Function,
}

/// Represents a symbol in the symbol table
/// 
/// A symbol contains all the information needed to identify and work with
/// a named entity in the language, including its name, kind, and type.
#[derive(Debug, Clone)]
pub struct Symbol {
    /// The name of the symbol as it appears in source code
    pub name: String,
    /// The kind of symbol (type, variable, or function)
    pub kind: SymbolKind,
    /// The type ID associated with this symbol
    pub type_id: TypeId,
}

/// A symbol table for managing symbols during compilation
/// 
/// The symbol table stores all named entities (variables, types, functions) 
/// in the current scope. It provides functionality to define new symbols
/// and look up existing ones by name. Each symbol is associated with its
/// kind and type information.
/// 
/// ### Example
/// ```
/// use slang_shared::{SymbolTable, SymbolKind};
/// use slang_types::TypeId;
/// 
/// let mut table = SymbolTable::new();
/// let type_id = TypeId::new(); // Example type ID
/// 
/// // Define a variable symbol
/// table.define("my_var".to_string(), SymbolKind::Variable, type_id).unwrap();
/// 
/// // Look up the symbol
/// let symbol = table.lookup("my_var").unwrap();
/// assert_eq!(symbol.name, "my_var");
/// ```
#[derive(Default)]
pub struct SymbolTable {
    /// Internal storage for symbols, mapping names to Symbol instances
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    /// Creates a new empty symbol table
    /// 
    /// ### Returns
    /// A new SymbolTable instance with no symbols defined
    /// 
    /// ### Example
    /// ```
    /// use slang_shared::SymbolTable;
    /// 
    /// let table = SymbolTable::new();
    /// assert!(table.lookup("nonexistent").is_none());
    /// ```
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    /// Defines a new symbol in the symbol table
    /// 
    /// Attempts to add a new symbol with the given name, kind, and type.
    /// If a symbol with the same name already exists in the current scope,
    /// returns an error with a descriptive message.
    /// 
    /// ### Arguments
    /// * `name` - The name of the symbol to define
    /// * `kind` - The kind of symbol (Type, Variable, or Function)
    /// * `type_id` - The type ID associated with this symbol
    /// 
    /// ### Returns
    /// * `Ok(())` if the symbol was successfully defined
    /// * `Err(String)` with an error message if the name is already taken
    /// 
    /// ### Example
    /// ```
    /// use slang_shared::{SymbolTable, SymbolKind};
    /// use slang_types::TypeId;
    /// 
    /// let mut table = SymbolTable::new();
    /// let type_id = TypeId::new();
    /// 
    /// // Define a new variable
    /// assert!(table.define("x".to_string(), SymbolKind::Variable, type_id.clone()).is_ok());
    /// 
    /// // Try to define the same name again - should fail
    /// assert!(table.define("x".to_string(), SymbolKind::Variable, type_id).is_err());
    /// ```
    pub fn define(
        &mut self,
        name: String,
        kind: SymbolKind,
        type_id: TypeId,
    ) -> Result<(), String> {
        let already_defined = self.symbols.get(&name);
        if let Some(def) = already_defined {
            if def.kind == SymbolKind::Type {
                return Err(format!(
                    "Type '{}' is already defined in the current scope.",
                    name
                ));
            } else if def.kind == SymbolKind::Function {
                return Err(format!(
                    "Function '{}' is already defined in the current scope.",
                    name
                ));
            } else if def.kind == SymbolKind::Variable {
                return Err(format!(
                    "Symbol '{}' is already defined in the current scope.",
                    name
                ));
            }
            return Err(format!(
                "Variable '{}' is already defined in the current scope.",
                name
            ));
        }
        self.symbols.insert(
            name.clone(),
            Symbol {
                name,
                kind,
                type_id,
            },
        );
        Ok(())
    }

    /// Looks up a symbol by name in the symbol table
    /// 
    /// Searches for a symbol with the given name and returns a reference
    /// to it if found. This is a read-only operation that does not modify
    /// the symbol table.
    /// 
    /// ### Arguments
    /// * `name` - The name of the symbol to look up
    /// 
    /// ### Returns
    /// * `Some(&Symbol)` if a symbol with the given name exists
    /// * `None` if no symbol with the given name is found
    /// 
    /// ### Example
    /// ```
    /// use slang_shared::{SymbolTable, SymbolKind};
    /// use slang_types::TypeId;
    /// 
    /// let mut table = SymbolTable::new();
    /// let type_id = TypeId::new();
    /// 
    /// table.define("my_function".to_string(), SymbolKind::Function, type_id).unwrap();
    /// 
    /// let symbol = table.lookup("my_function").unwrap();
    /// assert_eq!(symbol.kind, SymbolKind::Function);
    /// assert_eq!(symbol.name, "my_function");
    /// 
    /// assert!(table.lookup("nonexistent").is_none());
    /// ```
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}
