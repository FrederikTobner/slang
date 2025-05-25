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
            PrimitiveType::Bool => 1,
            _ => 0,
        }
    }
}

/// A unique identifier for a type in the type system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeId(pub usize);

impl Default for TypeId {
    fn default() -> Self {
        TypeId::new()
    }
}

impl TypeId {
    /// Creates a new unique type identifier
    pub fn new() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(PrimitiveType::Unknown as usize + 1);
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


/// Registry that stores all available types in the language
pub struct TypeRegistry {
    /// Map from TypeId to TypeInfo
    types: HashMap<TypeId, TypeInfo>,
}

impl TypeRegistry {
    /// Creates a new TypeRegistry with built-in types registered.
    pub fn new_instance() -> Self {
        let mut registry = TypeRegistry {
            types: HashMap::new(),
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
            self.register_primitive_type(ptype.name(), kind.clone(), TypeId(*ptype as usize));
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

        id
    }

        /// Registers a new type in the registry
    pub fn register_primitive_type(&mut self, name: &str, kind: TypeKind, id: TypeId) -> TypeId {
        let type_info = TypeInfo {
            id: id.clone(),
            name: name.to_string(),
            kind,
        };

        self.types.insert(id.clone(), type_info);

        id
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

