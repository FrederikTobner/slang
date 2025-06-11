use slang_types::types::TypeId;
use std::collections::HashMap;

/// Represents the specific data for each symbol kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolData {
    /// A type symbol (primitive types, structs, enums, etc.)
    Type,
    /// A variable symbol with mutability information
    Variable { is_mutable: bool },
    /// A function symbol (function declarations, built-in functions, etc.)
    Function,
}

/// Represents a symbol in the symbol table
///
/// A symbol contains all the information needed to identify and work with
/// a named entity in the language, including its name, specific data, and type.
#[derive(Debug, Clone)]
pub struct Symbol {
    /// The name of the symbol as it appears in source code
    pub name: String,
    /// The specific data for this symbol kind
    pub data: SymbolData,
    /// The type ID associated with this symbol
    pub type_id: TypeId,
}

impl Symbol {
    /// Returns the kind of this symbol for compatibility
    pub fn kind(&self) -> SymbolKind {
        match &self.data {
            SymbolData::Type => SymbolKind::Type,
            SymbolData::Variable { .. } => SymbolKind::Variable,
            SymbolData::Function => SymbolKind::Function,
        }
    }

    /// Returns whether this symbol is mutable (only meaningful for variables)
    pub fn is_mutable(&self) -> bool {
        match &self.data {
            SymbolData::Variable { is_mutable } => *is_mutable,
            _ => false,
        }
    }

    /// Returns true if this is a variable symbol
    pub fn is_variable(&self) -> bool {
        matches!(self.data, SymbolData::Variable { .. })
    }

    /// Returns true if this is a function symbol
    pub fn is_function(&self) -> bool {
        matches!(self.data, SymbolData::Function)
    }

    /// Returns true if this is a type symbol
    pub fn is_type(&self) -> bool {
        matches!(self.data, SymbolData::Type)
    }
}

/// Legacy enum for compatibility with existing code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Type,
    Variable,
    Function,
}

/// Represents a lexical scope containing symbols
#[derive(Debug, Clone)]
pub struct Scope {
    /// Map of symbol names to symbols in this scope
    symbols: HashMap<String, Symbol>,
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
/// use slang_shared::{SymbolTable, SymbolData};
/// use slang_types::TypeId;
///
/// let mut table = SymbolTable::new();
/// let type_id = TypeId::new(); // Example type ID
///
/// // Define a variable symbol
/// table.define("my_var".to_string(), SymbolData::Variable { is_mutable: true }, type_id).unwrap();
///
/// // Look up the symbol
/// let symbol = table.lookup("my_var").unwrap();
/// assert_eq!(symbol.name, "my_var");
/// ```
#[derive(Default)]
pub struct SymbolTable {
    /// Stack of scopes, with the innermost scope at the end
    scopes: Vec<Scope>,
}

impl SymbolTable {
    /// Creates a new symbol table with a global scope
    ///
    /// ### Returns
    /// A new SymbolTable instance with an empty global scope
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
            scopes: vec![Scope {
                symbols: HashMap::new(),
            }],
        }
    }

    /// Begins a new scope by pushing it onto the scope stack
    ///
    /// Used when entering a block, function, or other lexical scope.
    pub fn begin_scope(&mut self) {
        self.scopes.push(Scope {
            symbols: HashMap::new(),
        });
    }

    /// Ends the current scope by popping it from the scope stack
    ///
    /// Used when exiting a block, function, or other lexical scope.
    /// Will panic if attempting to end the global scope.
    pub fn end_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        } else {
            panic!("Cannot end the global scope");
        }
    }

    /// Defines a new symbol in the current (innermost) scope
    ///
    /// Attempts to add a new symbol with the given name, data, and type.
    /// If a symbol with the same name already exists in the current scope,
    /// returns an error with a descriptive message.
    ///
    /// ### Arguments
    /// * `name` - The name of the symbol to define
    /// * `data` - The specific data for this symbol kind
    /// * `type_id` - The type ID associated with this symbol
    ///
    /// ### Returns
    /// * `Ok(())` if the symbol was successfully defined
    /// * `Err(String)` with an error message if the name is already taken
    ///
    /// ### Example
    /// ```
    /// use slang_shared::{SymbolTable, SymbolData};
    /// use slang_types::TypeId;
    ///
    /// let mut table = SymbolTable::new();
    /// let type_id = TypeId::new();
    ///
    /// // Define a new variable
    /// assert!(table.define("x".to_string(), SymbolData::Variable { is_mutable: true }, type_id.clone()).is_ok());
    ///
    /// // Try to define the same name again - should fail
    /// assert!(table.define("x".to_string(), SymbolData::Variable { is_mutable: false }, type_id).is_err());
    /// ```
    pub fn define(
        &mut self,
        name: String,
        data: SymbolData,
        type_id: TypeId,
    ) -> Result<(), String> {
        // Check if symbol already exists in current scope
        if let Some(current_scope) = self.scopes.last() {
            if let Some(existing_symbol) = current_scope.symbols.get(&name) {
                let error_message = match (&existing_symbol.data, &data) {
                    (SymbolData::Type, _) => {
                        format!("Type '{}' is already defined in the current scope.", name)
                    }
                    (SymbolData::Function, _) => format!(
                        "Function '{}' is already defined in the current scope.",
                        name
                    ),
                    (SymbolData::Variable { .. }, _) => format!(
                        "Variable '{}' is already defined in the current scope.",
                        name
                    ),
                };
                return Err(error_message);
            }
        }

        // Add symbol to current scope
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.symbols.insert(
                name.clone(),
                Symbol {
                    name,
                    data,
                    type_id,
                },
            );
        }
        Ok(())
    }

    /// Looks up a symbol by name in all scopes, starting from innermost
    ///
    /// Searches for a symbol with the given name starting from the innermost
    /// (current) scope and working outward. Returns a reference to the first
    /// matching symbol found.
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
    /// use slang_shared::{SymbolTable, SymbolData};
    /// use slang_types::TypeId;
    ///
    /// let mut table = SymbolTable::new();
    /// let type_id = TypeId::new();
    ///
    /// table.define("my_function".to_string(), SymbolData::Function, type_id).unwrap();
    ///
    /// let symbol = table.lookup("my_function").unwrap();
    /// assert!(matches!(symbol.data, SymbolData::Function));
    /// assert_eq!(symbol.name, "my_function");
    ///
    /// assert!(table.lookup("nonexistent").is_none());
    /// ```
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol);
            }
        }
        None
    }
}
