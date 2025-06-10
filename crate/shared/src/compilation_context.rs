use crate::{Symbol, SymbolKind, SymbolTable};
use crate::symbol_table::SymbolData;
use slang_types::{PrimitiveType, StructType, TypeId, TypeInfo, TypeKind, TypeRegistry, FunctionType};

/// Compilation context that owns the type registry and symbol table
pub struct CompilationContext {
    /// The type registry that stores all types
    type_registry: TypeRegistry,
    /// The symbol table that stores all symbols (variables, types, functions)
    symbol_table: SymbolTable,
}

impl Default for CompilationContext {
    fn default() -> Self {
        CompilationContext::new()
    }
}

impl CompilationContext {
    /// Creates a new compilation context with a type registry and symbol table
    ///
    /// Initializes the context with all primitive types registered in both the type registry
    /// and symbol table. This includes boolean, integer, float, string, and unspecified types.
    ///
    /// ### Returns
    /// A new CompilationContext instance ready for compilation
    pub fn new() -> Self {
        let type_registry = TypeRegistry::new_instance();
        let mut symbol_table = SymbolTable::new();

        let mut define_primitive = |ptype: PrimitiveType| {
            let type_id = TypeId::from_primitive(ptype);
            symbol_table
                .define(ptype.name().to_string(), SymbolData::Type, type_id.clone())
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to define primitive type symbol for '{}'",
                        ptype.name()
                    )
                });
            type_id
        };

        define_primitive(PrimitiveType::Bool);
        define_primitive(PrimitiveType::I32);
        define_primitive(PrimitiveType::I64);
        define_primitive(PrimitiveType::U32);
        define_primitive(PrimitiveType::U64);
        define_primitive(PrimitiveType::F32);
        define_primitive(PrimitiveType::F64);
        define_primitive(PrimitiveType::String);
        define_primitive(PrimitiveType::UnspecifiedInt);
        define_primitive(PrimitiveType::UnspecifiedFloat);
        define_primitive(PrimitiveType::Unknown);

        CompilationContext {
            type_registry,
            symbol_table,
        }
    }

    /// Gets type information for a given type ID
    ///
    /// ### Arguments
    /// * `id` - The type ID to look up
    ///
    /// ### Returns
    /// An optional reference to the TypeInfo if the type exists
    pub fn get_type_info(&self, id: &TypeId) -> Option<&TypeInfo> {
        self.type_registry.get_type_info(id)
    }

    /// Gets the name of a type from its TypeId
    ///
    /// ### Arguments
    /// * `type_id` - The type ID to get the name for
    ///
    /// ### Returns
    /// The name of the type as a String, or a debug representation if the type is unknown
    pub fn get_type_name(&self, type_id: &TypeId) -> String {
        self.type_registry
            .get_type_info(type_id)
            .map(|t| t.name.clone())
            .unwrap_or_else(|| format!("UnknownTypeId({:?})", type_id.0))
    }

    /// Gets the primitive type corresponding to a given type ID
    ///
    /// ### Arguments
    /// * `id` - The type ID to look up
    ///
    /// ### Returns
    /// An optional PrimitiveType if the type ID corresponds to a primitive type
    pub fn get_primitive_type_from_id(&self, id: &TypeId) -> Option<PrimitiveType> {
        self.type_registry.get_primitive_type(id)
    }

    /// Checks if a type ID corresponds to a primitive type
    ///
    /// ### Arguments
    /// * `id` - The type ID to check
    ///
    /// ### Returns
    /// True if the type is a primitive type, false otherwise
    pub fn is_primitive_type(&self, id: &TypeId) -> bool {
        self.type_registry.is_primitive_type(id)
    }

    /// Checks if a type fulfills a given predicate function
    ///
    /// ### Arguments
    /// * `type_id` - The type ID to check
    /// * `predicate` - A function that takes TypeInfo and returns a boolean
    ///
    /// ### Returns
    /// True if the type exists and satisfies the predicate, false otherwise
    pub fn type_fulfills<F>(&self, type_id: &TypeId, predicate: F) -> bool
    where
        F: Fn(&TypeInfo) -> bool,
    {
        self.get_type_info(type_id).is_some_and(predicate)
    }

    /// Checks if a type ID corresponds to a numeric type (integer or float)
    ///
    /// ### Arguments
    /// * `type_id` - The type ID to check
    ///
    /// ### Returns
    /// True if the type is numeric (integer or float), false otherwise
    pub fn is_numeric_type(&self, type_id: &TypeId) -> bool {
        self.get_primitive_type_from_id(type_id)
            .is_some_and(|pt| pt.is_numeric())
    }

    /// Checks if a type ID corresponds to an integer type
    ///
    /// ### Arguments
    /// * `type_id` - The type ID to check
    ///
    /// ### Returns
    /// True if the type is an integer type (signed or unsigned), false otherwise
    pub fn is_integer_type(&self, type_id: &TypeId) -> bool {
        self.get_primitive_type_from_id(type_id)
            .is_some_and(|pt| pt.is_integer())
    }

    /// Checks if a type ID corresponds to a floating-point type
    ///
    /// ### Arguments
    /// * `type_id` - The type ID to check
    ///
    /// ### Returns
    /// True if the type is a floating-point type (f32 or f64), false otherwise
    pub fn is_float_type(&self, type_id: &TypeId) -> bool {
        self.get_primitive_type_from_id(type_id)
            .is_some_and(|pt| pt.is_float())
    }

    /// Checks if a type ID corresponds to a signed integer type
    ///
    /// ### Arguments
    /// * `type_id` - The type ID to check
    ///
    /// ### Returns
    /// True if the type is a signed integer type (i32 or i64), false otherwise
    pub fn is_signed_integer_type(&self, type_id: &TypeId) -> bool {
        self.get_primitive_type_from_id(type_id)
            .is_some_and(|pt| pt.is_signed_integer())
    }

    /// Checks if a type ID corresponds to an unsigned integer type
    ///
    /// ### Arguments
    /// * `type_id` - The type ID to check
    ///
    /// ### Returns
    /// True if the type is an unsigned integer type (u32 or u64), false otherwise
    pub fn is_unsigned_integer_type(&self, type_id: &TypeId) -> bool {
        self.get_primitive_type_from_id(type_id)
            .is_some_and(|pt| pt.is_unsigned_integer())
    }

    /// Gets the bit width of a type
    ///
    /// ### Arguments
    /// * `type_id` - The type ID to get the bit width for
    ///
    /// ### Returns
    /// The bit width of the type, or 0 if the type is not a primitive type
    pub fn get_bit_width(&self, type_id: &TypeId) -> u8 {
        self.get_primitive_type_from_id(type_id)
            .map_or(0, |pt| pt.bit_width())
    }

    /// Checks if an integer value is within the valid range for a given type
    ///
    /// ### Arguments
    /// * `value` - The integer value to check
    /// * `type_id` - The type ID to check the value against
    ///
    /// ### Returns
    /// True if the value is within the valid range for the type, false otherwise
    pub fn check_value_in_range(&self, value: &i64, type_id: &TypeId) -> bool {
        self.type_registry.check_value_in_range(value, type_id)
    }

    /// Checks if a floating-point value is within the valid range for a given type
    ///
    /// ### Arguments
    /// * `value` - The floating-point value to check
    /// * `type_id` - The type ID to check the value against
    ///
    /// ### Returns
    /// True if the value is within the valid range for the type, false otherwise
    pub fn check_float_value_in_range(&self, value: &f64, type_id: &TypeId) -> bool {
        self.type_registry
            .check_float_value_in_range(value, type_id)
    }

    /// Defines a symbol in the symbol table
    ///
    /// ### Arguments
    /// * `name` - The name of the symbol
    /// * `kind` - The kind of symbol (variable, type, function)
    /// * `type_id` - The type ID associated with the symbol
    /// * `is_mutable` - Whether the symbol is mutable (only relevant for variables)
    ///
    /// ### Returns
    /// A Result indicating success or an error message if the symbol cannot be defined
    pub fn define_symbol(
        &mut self,
        name: String,
        kind: SymbolKind,
        type_id: TypeId,
        is_mutable: bool,
    ) -> Result<(), String> {
        let data = match kind {
            SymbolKind::Type => SymbolData::Type,
            SymbolKind::Variable => SymbolData::Variable { is_mutable },
            SymbolKind::Function => SymbolData::Function,
        };
        self.symbol_table.define(name, data, type_id)
    }

    /// Looks up a symbol in the symbol table by name
    ///
    /// ### Arguments
    /// * `name` - The name of the symbol to look up
    ///
    /// ### Returns
    /// An optional reference to the Symbol if found, None otherwise
    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbol_table.lookup(name)
    }

    /// Registers a custom type with the given name and type kind
    ///
    /// ### Arguments
    /// * `name` - The name of the custom type
    /// * `type_kind` - The kind of type to register (struct, enum, etc.)
    ///
    /// ### Returns
    /// A Result containing the TypeId of the registered type or an error message if the name is already defined
    pub fn register_custom_type(
        &mut self,
        name: &str,
        type_kind: TypeKind,
    ) -> Result<TypeId, String> {
        if self.symbol_table.lookup(name).is_some() {
            return Err(format!("Symbol '{}' is already defined.", name));
        }

        let type_id = self.type_registry.register_type(name, type_kind);
        self.symbol_table
            .define(name.to_string(), SymbolData::Type, type_id.clone())?;
        Ok(type_id)
    }

    /// Registers a new struct type with the given name and fields
    ///
    /// ### Arguments
    /// * `name` - The name of the struct type
    /// * `fields` - A vector of tuples containing field names and their type IDs
    ///
    /// ### Returns
    /// A Result containing the TypeId of the registered struct type or an error message
    pub fn register_struct_type(
        &mut self,
        name: String,
        fields: Vec<(String, TypeId)>,
    ) -> Result<TypeId, String> {
        let struct_type = StructType::new(name.clone(), fields);
        let type_kind = TypeKind::Struct(struct_type);
        self.register_custom_type(&name, type_kind)
    }

    /// Registers a function type and returns its TypeId
    pub fn register_function_type(&mut self, param_types: Vec<TypeId>, return_type: TypeId) -> TypeId {
        self.type_registry.register_function_type(param_types, return_type)
    }

    /// Checks if a type is a function type
    pub fn is_function_type(&self, type_id: &TypeId) -> bool {
        if let Some(type_info) = self.type_registry.get_type_info(type_id) {
            matches!(type_info.kind, TypeKind::Function(_))
        } else {
            false
        }
    }

    /// Gets function type information
    pub fn get_function_type(&self, type_id: &TypeId) -> Option<&FunctionType> {
        if let Some(type_info) = self.type_registry.get_type_info(type_id) {
            if let TypeKind::Function(ref function_type) = type_info.kind {
                Some(function_type)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Begins a new scope by calling the symbol table
    /// Used when entering a block, function, or other lexical scope.
    pub fn begin_scope(&mut self) {
        self.symbol_table.begin_scope();
    }

    /// Ends the current scope by calling the symbol table
    /// Used when exiting a block, function, or other lexical scope.
    pub fn end_scope(&mut self) {
        self.symbol_table.end_scope();
    }
}
