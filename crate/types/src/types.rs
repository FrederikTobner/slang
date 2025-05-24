use slang_derive::NamedEnum;
use std::collections::HashMap;

// Type name constants
pub const TYPE_NAME_I32: &str = PrimitiveType::I32.name();
pub const TYPE_NAME_I64: &str = PrimitiveType::I64.name();
pub const TYPE_NAME_U32: &str = PrimitiveType::U32.name();
pub const TYPE_NAME_U64: &str = PrimitiveType::U64.name();
pub const TYPE_NAME_F32: &str = PrimitiveType::F32.name();
pub const TYPE_NAME_F64: &str = PrimitiveType::F64.name();
pub const TYPE_NAME_BOOL: &str = PrimitiveType::Bool.name();
pub const TYPE_NAME_STRING: &str = PrimitiveType::String.name();
pub const TYPE_NAME_INT: &str = PrimitiveType::UnspecifiedInt.name();
pub const TYPE_NAME_FLOAT: &str = PrimitiveType::UnspecifiedFloat.name();
pub const TYPE_NAME_UNKNOWN: &str = PrimitiveType::Unknown.name();

/// Represents all primitive types in the language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, NamedEnum)]
pub enum PrimitiveType {
    /// 32-bit signed integer
    I32,
    /// 64-bit signed integer
    I64,
    /// 32-bit unsigned integer
    U32,
    /// 64-bit unsigned integer
    U64,
    /// 32-bit floating point
    F32,
    /// 64-bit floating point
    F64,
    /// Boolean type
    Bool,
    /// String type
    String,
    /// Unspecified integer type (for literals)
    #[name = "int"]
    UnspecifiedInt,
    /// Unspecified float type (for literals)
    #[name = "float"]
    UnspecifiedFloat,
    /// Unknown type
    Unknown,
}

impl PrimitiveType {
    /// Get the TypeId for this primitive type using the CompilationContext.
    pub fn get_type_id(&self, context: &CompilationContext) -> TypeId {
        match self {
            PrimitiveType::I32 => context.i32_type(),
            PrimitiveType::I64 => context.i64_type(),
            PrimitiveType::U32 => context.u32_type(),
            PrimitiveType::U64 => context.u64_type(),
            PrimitiveType::F32 => context.f32_type(),
            PrimitiveType::F64 => context.f64_type(),
            PrimitiveType::Bool => context.bool_type(),
            PrimitiveType::String => context.string_type(),
            PrimitiveType::UnspecifiedInt => context.unspecified_int_type(),
            PrimitiveType::UnspecifiedFloat => context.unspecified_float_type(),
            PrimitiveType::Unknown => context.unknown_type(),
        }
    }

    /// Check if this is a numeric type (integer or float)
    pub fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    /// Check if this is an integer type
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            PrimitiveType::I32
                | PrimitiveType::I64
                | PrimitiveType::U32
                | PrimitiveType::U64
                | PrimitiveType::UnspecifiedInt
        )
    }

    /// Check if this is a float type
    pub fn is_float(&self) -> bool {
        matches!(
            self,
            PrimitiveType::F32 | PrimitiveType::F64 | PrimitiveType::UnspecifiedFloat
        )
    }

    /// Check if this is a signed integer type
    pub fn is_signed_integer(&self) -> bool {
        matches!(
            self,
            PrimitiveType::I32 | PrimitiveType::I64 | PrimitiveType::UnspecifiedInt
        )
    }

    /// Check if this is an unsigned integer type
    pub fn is_unsigned_integer(&self) -> bool {
        matches!(self, PrimitiveType::U32 | PrimitiveType::U64)
    }

    /// Get the bit width of this type (0 for unspecified types)
    pub fn bit_width(&self) -> u8 {
        match self {
            PrimitiveType::I32 | PrimitiveType::U32 | PrimitiveType::F32 => 32,
            PrimitiveType::I64 | PrimitiveType::U64 | PrimitiveType::F64 => 64,
            _ => 0,
        }
    }
}

/// A unique identifier for a type in the type system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeId(usize);

impl Default for TypeId {
    fn default() -> Self {
        TypeId::new()
    }
}

impl TypeId {
    /// Creates a new unique type identifier
    pub fn new() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        TypeId(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

/// Represents the different kinds of types in the language
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TypeKind {
    /// Integer types (signed/unsigned, different bit widths)
    Integer(IntegerType),
    /// Floating point types
    Float(FloatType),
    /// String type
    String,
    /// Boolean type
    Boolean,
    /// Struct type with fields
    Struct(StructType),
    /// Unknown or not yet determined type
    Unknown,
}

/// Represents an integer type with its properties
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IntegerType {
    /// Whether the integer is signed or unsigned
    pub signed: bool,
    /// The number of bits (e.g., 32 for i32)
    pub bits: u8,
    /// Whether this is an unspecified integer (used for literals without explicit type)
    pub is_unspecified: bool,
}

/// Represents a floating point type with its properties
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FloatType {
    /// The number of bits (e.g., 64 for f64)
    pub bits: u8,
    /// Whether this is an unspecified float (used for literals without explicit type)
    pub is_unspecified: bool,
}

/// Represents a struct type with its fields
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StructType {
    /// Name of the struct
    pub name: String,
    /// Fields of the struct with their names and types
    pub fields: Vec<(String, TypeId)>,
}

impl StructType {
    /// Creates a new StructType.
    pub fn new(name: String, fields: Vec<(String, TypeId)>) -> Self {
        StructType { name, fields }
    }
}

/// Contains all information about a specific type
#[derive(Debug)]
#[allow(dead_code)]
pub struct TypeInfo {
    /// Unique identifier for this type
    pub id: TypeId,
    /// Name of the type
    pub name: String,
    /// The kind of type (integer, float, string, etc.)
    pub kind: TypeKind,
}

/// Represents the kind of a symbol
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Type,
    Variable,
    Function,
}

/// Represents a symbol in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub type_id: TypeId,
}

/// A symbol table for managing symbols
#[derive(Default)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    /// Defines a new symbol. Returns an error if the name is already taken in the current scope.
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
            } else if def.kind == SymbolKind::Variable
            {
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

    /// Looks up a symbol by name.
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

/// Registry that stores all available types in the language
pub struct TypeRegistry {
    /// Map from TypeId to TypeInfo
    types: HashMap<TypeId, TypeInfo>,
    /// Map from type names to TypeId for quick lookup
    type_names: HashMap<String, TypeId>,
}

impl TypeRegistry {
    /// Creates a new TypeRegistry with built-in types registered.
    fn new_instance() -> Self {
        let mut registry = TypeRegistry {
            types: HashMap::new(),
            type_names: HashMap::new(),
        };
        registry.register_built_in_types();
        registry
    }

    /// Registers all built-in types in the type registry
    fn register_built_in_types(&mut self) {
        let types_to_register: &[(PrimitiveType, TypeKind)] = &[
            (
                PrimitiveType::I32,
                TypeKind::Integer(IntegerType {
                    signed: true,
                    bits: 32,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::I64,
                TypeKind::Integer(IntegerType {
                    signed: true,
                    bits: 64,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::U32,
                TypeKind::Integer(IntegerType {
                    signed: false,
                    bits: 32,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::U64,
                TypeKind::Integer(IntegerType {
                    signed: false,
                    bits: 64,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::UnspecifiedInt,
                TypeKind::Integer(IntegerType {
                    signed: true,
                    bits: 0,
                    is_unspecified: true,
                }),
            ),
            (
                PrimitiveType::F32,
                TypeKind::Float(FloatType {
                    bits: 32,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::F64,
                TypeKind::Float(FloatType {
                    bits: 64,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::UnspecifiedFloat,
                TypeKind::Float(FloatType {
                    bits: 0,
                    is_unspecified: true,
                }),
            ),
            (PrimitiveType::String, TypeKind::String),
            (PrimitiveType::Bool, TypeKind::Boolean),
            (PrimitiveType::Unknown, TypeKind::Unknown),
        ];

        for (ptype, kind) in types_to_register {
            self.register_type(ptype.name(), kind.clone());
        }
    }

    /// Registers a new type in the registry
    pub fn register_type(&mut self, name: &str, kind: TypeKind) -> TypeId {
        let id = TypeId::new();
        let type_info = TypeInfo {
            id: id.clone(),
            name: name.to_string(),
            kind,
        };

        self.types.insert(id.clone(), type_info);
        self.type_names.insert(name.to_string(), id.clone());

        id
    }

    /// Looks up a type by name
    /// 
    /// ### Arguments
    /// * `name` - The name of the type to look up
    /// 
    /// ### Returns
    /// An Option containing the TypeId if found, or None if not found
    pub fn get_type_by_name(&self, name: &str) -> Option<&TypeId> {
        self.type_names.get(name)
    }

    /// Gets type information for a given TypeId
    /// 
    /// ### Arguments
    /// * `id` - The TypeId to look up
    /// 
    /// ### Returns
    /// An Option containing the TypeInfo if found, or None if not found
    pub fn get_type_info(&self, id: &TypeId) -> Option<&TypeInfo> {
        self.types.get(id)
    }

    /// Try to get the primitive type for a given TypeId
    /// 
    /// ### Arguments
    /// * `id` - The TypeId to look up
    /// 
    /// ### Returns
    /// An Option containing the PrimitiveType if found, or None if not found
    pub fn get_primitive_type(&self, id: &TypeId) -> Option<PrimitiveType> {
        self.get_type_info(id)
            .and_then(|info| PrimitiveType::from_str(&info.name))
    }

    /// Check if a type is a primitive type
    /// 
    /// ### Arguments
    /// * `id` - The TypeId to check
    /// 
    /// ### Returns
    /// A boolean indicating whether the type is a primitive type
    pub fn is_primitive_type(&self, id: &TypeId) -> bool {
        self.get_primitive_type(id).is_some()
    }

    /// Checks if a value is within the valid range for a given type
    /// 
    /// ### Arguments
    /// * `value` - The value to check
    /// * `type_id` - The TypeId of the type to check against
    /// 
    /// ### Returns
    /// A boolean indicating whether the value is within the valid range
    pub fn check_value_in_range(&self, value: &i64, type_id: &TypeId) -> bool {
        let type_info = match self.get_type_info(type_id) {
            Some(info) => info,
            None => return false,
        };

        match &type_info.kind {
            TypeKind::Integer(int_type) => match (int_type.signed, int_type.bits) {
                (true, 32) => *value >= i32::MIN as i64 && *value <= i32::MAX as i64,
                (true, 64) => true,
                (false, 32) => *value >= 0 && *value <= u32::MAX as i64,
                (false, 64) => *value >= 0,
                _ => false,
            },
            TypeKind::Float(float_type) => match float_type.bits {
                32 => *value >= f32::MIN as i64 && *value <= f32::MAX as i64,
                64 => true,
                _ => *value >= f64::MIN as i64 && *value <= f64::MAX as i64,
            },
            _ => false,
        }
    }

    /// Checks if a float value is within the valid range for a given type
    /// 
    /// ### Arguments
    /// * `value` - The float value to check
    /// * `type_id` - The TypeId of the type to check against
    /// 
    /// ### Returns
    /// A boolean indicating whether the float value is within the valid range
    pub fn check_float_value_in_range(&self, value: &f64, type_id: &TypeId) -> bool {
        let type_info = match self.get_type_info(type_id) {
            Some(info) => info,
            None => return false,
        };

        match &type_info.kind {
            TypeKind::Float(float_type) => match float_type.bits {
                32 => *value >= f32::MIN as f64 && *value <= f32::MAX as f64,
                64 => true,
                _ => false,
            },
            _ => false,
        }
    }
}

/// Compilation context that owns the type registry and symbol table
pub struct CompilationContext {
    /// The type registry that stores all types
    type_registry: TypeRegistry,
    symbol_table: SymbolTable,
    bool_type_id: TypeId,
    i32_type_id: TypeId,
    i64_type_id: TypeId,
    u32_type_id: TypeId,
    u64_type_id: TypeId,
    f32_type_id: TypeId,
    f64_type_id: TypeId,
    string_type_id: TypeId,
    unspecified_int_type_id: TypeId,
    unspecified_float_type_id: TypeId,
    unknown_type_id: TypeId,
}

impl Default for CompilationContext {
    fn default() -> Self {
        CompilationContext::new()
    }
}

impl CompilationContext {
    pub fn new() -> Self {
        let type_registry = TypeRegistry::new_instance();
        let mut symbol_table = SymbolTable::new();

        let mut cache_and_define_symbol = |ptype: PrimitiveType| {
            let type_id = type_registry
                .get_type_by_name(ptype.name())
                .unwrap_or_else(|| {
                    panic!(
                        "Primitive type '{}' not found in registry during context init",
                        ptype.name()
                    )
                })
                .clone();
            symbol_table
                .define(ptype.name().to_string(), SymbolKind::Type, type_id.clone())
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to define primitive type symbol for '{}'",
                        ptype.name()
                    )
                });
            type_id
        };

        let bool_type_id = cache_and_define_symbol(PrimitiveType::Bool);
        let i32_type_id = cache_and_define_symbol(PrimitiveType::I32);
        let i64_type_id = cache_and_define_symbol(PrimitiveType::I64);
        let u32_type_id = cache_and_define_symbol(PrimitiveType::U32);
        let u64_type_id = cache_and_define_symbol(PrimitiveType::U64);
        let f32_type_id = cache_and_define_symbol(PrimitiveType::F32);
        let f64_type_id = cache_and_define_symbol(PrimitiveType::F64);
        let string_type_id = cache_and_define_symbol(PrimitiveType::String);
        let unspecified_int_type_id = cache_and_define_symbol(PrimitiveType::UnspecifiedInt);
        let unspecified_float_type_id = cache_and_define_symbol(PrimitiveType::UnspecifiedFloat);
        let unknown_type_id = cache_and_define_symbol(PrimitiveType::Unknown);

        CompilationContext {
            type_registry,
            symbol_table,
            bool_type_id,
            i32_type_id,
            i64_type_id,
            u32_type_id,
            u64_type_id,
            f32_type_id,
            f64_type_id,
            string_type_id,
            unspecified_int_type_id,
            unspecified_float_type_id,
            unknown_type_id,
        }
    }

    pub fn bool_type(&self) -> TypeId {
        self.bool_type_id.clone()
    }
    pub fn i32_type(&self) -> TypeId {
        self.i32_type_id.clone()
    }
    pub fn i64_type(&self) -> TypeId {
        self.i64_type_id.clone()
    }
    pub fn u32_type(&self) -> TypeId {
        self.u32_type_id.clone()
    }
    pub fn u64_type(&self) -> TypeId {
        self.u64_type_id.clone()
    }
    pub fn f32_type(&self) -> TypeId {
        self.f32_type_id.clone()
    }
    pub fn f64_type(&self) -> TypeId {
        self.f64_type_id.clone()
    }
    pub fn string_type(&self) -> TypeId {
        self.string_type_id.clone()
    }
    pub fn unspecified_int_type(&self) -> TypeId {
        self.unspecified_int_type_id.clone()
    }
    pub fn unspecified_float_type(&self) -> TypeId {
        self.unspecified_float_type_id.clone()
    }
    pub fn unknown_type(&self) -> TypeId {
        self.unknown_type_id.clone()
    }

    pub fn get_type_info(&self, id: &TypeId) -> Option<&TypeInfo> {
        self.type_registry.get_type_info(id)
    }

    pub fn get_type_name(&self, type_id: &TypeId) -> String {
        self.type_registry
            .get_type_info(type_id)
            .map(|t| t.name.clone())
            .unwrap_or_else(|| format!("UnknownTypeId({:?})", type_id.0))
    }

    pub fn get_primitive_type_from_id(&self, id: &TypeId) -> Option<PrimitiveType> {
        self.type_registry.get_primitive_type(id)
    }

    pub fn is_primitive_type(&self, id: &TypeId) -> bool {
        self.type_registry.is_primitive_type(id)
    }

    pub fn type_fulfills<F>(&self, type_id: &TypeId, predicate: F) -> bool
    where
        F: Fn(&TypeInfo) -> bool,
    {
        self.get_type_info(type_id).is_some_and(predicate)
    }

    pub fn is_numeric_type(&self, type_id: &TypeId) -> bool {
        self.get_primitive_type_from_id(type_id)
            .is_some_and(|pt| pt.is_numeric())
    }

    pub fn is_integer_type(&self, type_id: &TypeId) -> bool {
        self.get_primitive_type_from_id(type_id)
            .is_some_and(|pt| pt.is_integer())
    }

    pub fn is_float_type(&self, type_id: &TypeId) -> bool {
        self.get_primitive_type_from_id(type_id)
            .is_some_and(|pt| pt.is_float())
    }

    pub fn is_signed_integer_type(&self, type_id: &TypeId) -> bool {
        self.get_primitive_type_from_id(type_id)
            .is_some_and(|pt| pt.is_signed_integer())
    }

    pub fn is_unsigned_integer_type(&self, type_id: &TypeId) -> bool {
        self.get_primitive_type_from_id(type_id)
            .is_some_and(|pt| pt.is_unsigned_integer())
    }

    pub fn get_bit_width(&self, type_id: &TypeId) -> u8 {
        self.get_primitive_type_from_id(type_id)
            .map_or(0, |pt| pt.bit_width())
    }

    pub fn check_value_in_range(&self, value: &i64, type_id: &TypeId) -> bool {
        self.type_registry.check_value_in_range(value, type_id)
    }

    pub fn check_float_value_in_range(&self, value: &f64, type_id: &TypeId) -> bool {
        self.type_registry
            .check_float_value_in_range(value, type_id)
    }

    pub fn define_symbol(
        &mut self,
        name: String,
        kind: SymbolKind,
        type_id: TypeId,
    ) -> Result<(), String> {
        self.symbol_table.define(name, kind, type_id)
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbol_table.lookup(name)
    }

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
            .define(name.to_string(), SymbolKind::Type, type_id.clone())?;
        Ok(type_id)
    }

    pub fn register_struct_type(
        &mut self,
        name: String,
        fields: Vec<(String, TypeId)>,
    ) -> Result<TypeId, String> {
        let struct_type = StructType::new(name.clone(), fields);
        let type_kind = TypeKind::Struct(struct_type);
        self.register_custom_type(&name, type_kind)
    }
}
