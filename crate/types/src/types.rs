use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, RwLock};
use slang_derive::NamedEnum;

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
    #[name = "unknown"]
    Unknown,
}

impl PrimitiveType {

    /// Get the TypeId for this primitive type
    pub fn get_type_id(&self) -> TypeId {
        TYPE_REGISTRY
            .read()
            .unwrap()
            .get_type_by_name(self.name())
            .unwrap()
            .clone()
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
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        TypeId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub fn get_type_name(type_id: &TypeId) -> String {
    TYPE_REGISTRY
        .read()
        .unwrap()
        .get_type_info(type_id)
        .map(|t| t.name.clone())
        .unwrap_or_else(|| format!("{:?}", type_id))
}

pub fn type_fullfills<F>(type_id: &TypeId, predicate: F) -> bool
where
    F: Fn(&TypeInfo) -> bool,
{
    TYPE_REGISTRY
        .read()
        .unwrap()
        .get_type_info(type_id)
        .is_some_and(predicate)
}

/// Returns the TypeId for a primitive type
pub fn primitive_type(ptype: PrimitiveType) -> TypeId {
    ptype.get_type_id()
}

/// Returns the TypeId for booleans
pub fn bool_type() -> TypeId {
    PrimitiveType::Bool.get_type_id()
}

/// Returns the TypeId for i32 integers
pub fn i32_type() -> TypeId {
    PrimitiveType::I32.get_type_id()
}

/// Returns the TypeId for i64 integers
pub fn i64_type() -> TypeId {
    PrimitiveType::I64.get_type_id()
}

/// Returns the TypeId for u32 integers
pub fn u32_type() -> TypeId {
    PrimitiveType::U32.get_type_id()
}

/// Returns the TypeId for u64 integers
pub fn u64_type() -> TypeId {
    PrimitiveType::U64.get_type_id()
}

/// Returns the TypeId for f32 floating points
pub fn f32_type() -> TypeId {
    PrimitiveType::F32.get_type_id()
}

/// Returns the TypeId for f64 floating points
pub fn f64_type() -> TypeId {
    PrimitiveType::F64.get_type_id()
}

/// Returns the TypeId for unspecified floats (used for float literals without suffix)
pub fn unspecified_float_type() -> TypeId {
    PrimitiveType::UnspecifiedFloat.get_type_id()
}

/// Returns the TypeId for strings
pub fn string_type() -> TypeId {
    PrimitiveType::String.get_type_id()
}

/// Returns the TypeId for unspecified integers (used for integer literals)
pub fn unspecified_int_type() -> TypeId {
    PrimitiveType::UnspecifiedInt.get_type_id()
}

/// Returns the TypeId for unknown types
pub fn unknown_type() -> TypeId {
    PrimitiveType::Unknown.get_type_id()
}

/// Check if a type is a numeric type (integer or float)
pub fn is_numeric_type(type_id: &TypeId) -> bool {
    TYPE_REGISTRY
        .read()
        .unwrap()
        .get_primitive_type(type_id)
        .is_some_and(|pt| pt.is_numeric())
}

/// Check if a type is an integer type
pub fn is_integer_type(type_id: &TypeId) -> bool {
    TYPE_REGISTRY
        .read()
        .unwrap()
        .get_primitive_type(type_id)
        .is_some_and(|pt| pt.is_integer())
}

/// Check if a type is a float type
pub fn is_float_type(type_id: &TypeId) -> bool {
    TYPE_REGISTRY
        .read()
        .unwrap()
        .get_primitive_type(type_id)
        .is_some_and(|pt| pt.is_float())
}

/// Check if a type is a signed integer type
pub fn is_signed_integer_type(type_id: &TypeId) -> bool {
    TYPE_REGISTRY
        .read()
        .unwrap()
        .get_primitive_type(type_id)
        .is_some_and(|pt| pt.is_signed_integer())
}

/// Check if a type is an unsigned integer type
pub fn is_unsigned_integer_type(type_id: &TypeId) -> bool {
    TYPE_REGISTRY
        .read()
        .unwrap()
        .get_primitive_type(type_id)
        .is_some_and(|pt| pt.is_unsigned_integer())
}

/// Try to get the primitive type for a TypeId
pub fn get_primitive_type(type_id: &TypeId) -> Option<PrimitiveType> {
    TYPE_REGISTRY.read().unwrap().get_primitive_type(type_id)
}

/// Get the bit width of a type (0 for non-numeric or unspecified types)
pub fn get_bit_width(type_id: &TypeId) -> u8 {
    get_primitive_type(type_id).map_or(0, |pt| pt.bit_width())
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

pub static TYPE_REGISTRY: LazyLock<RwLock<TypeRegistry>> = LazyLock::new(|| {
    let registry = TypeRegistry::new();
    RwLock::new(registry)
});

// Thread-local storage for the type registry and commonly used types
thread_local! {
    /// Pre-defined type for booleans
    pub static BOOL_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_BOOL).unwrap().clone());
    /// Pre-defined type for i32 integers
    pub static I32_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_I32).unwrap().clone());
    /// Pre-defined type for i64 integers
    pub static I64_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_I64).unwrap().clone());
    /// Pre-defined type for u32 integers
    pub static U32_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_U32).unwrap().clone());
    /// Pre-defined type for u64 integers
    pub static U64_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_U64).unwrap().clone());
    /// Pre-defined type for f32 floating points
    pub static F32_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_F32).unwrap().clone());
    /// Pre-defined type for f64 floating points
    pub static F64_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_F64).unwrap().clone());
    /// Pre-defined type for strings
    pub static STRING_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_STRING).unwrap().clone());
    /// Pre-defined type for unspecified integers (used for integer literals)
    pub static UNSPECIFIED_INT_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_INT).unwrap().clone());
    /// Pre-defined type for unspecified floats (used for float literals without suffix)
    pub static UNSPECIFIED_FLOAT_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_FLOAT).unwrap().clone());
    /// Pre-defined type for unknown types
    pub static UNKNOWN_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.read().unwrap().get_type_by_name(TYPE_NAME_UNKNOWN).unwrap().clone());
}

/// Registry that stores all available types in the language
pub struct TypeRegistry {
    /// Map from TypeId to TypeInfo
    types: HashMap<TypeId, TypeInfo>,
    /// Map from type names to TypeId for quick lookup
    type_names: HashMap<String, TypeId>,
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeRegistry {
    /// Creates a new TypeRegistry with built-in types registered
    pub fn new() -> Self {
        let mut registry = TypeRegistry {
            types: HashMap::new(),
            type_names: HashMap::new(),
        };

        // Register the built-in types
        registry.register_built_in_types();
        registry
    }

    /// Registers all built-in types in the type registry
    fn register_built_in_types(&mut self) {
        let types_to_register: &[(PrimitiveType, TypeKind)] = &[
            // Integer types
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
            // Float types
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
            // Other types
            (PrimitiveType::String, TypeKind::String),
            (PrimitiveType::Bool, TypeKind::Boolean),
            (PrimitiveType::Unknown, TypeKind::Unknown),
        ];

        for (ptype, kind) in types_to_register {
            self.register_type(ptype.name(), kind.clone());
        }
    }

    /// Registers a new type in the registry
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the type
    /// * `kind` - The kind of type to register
    ///
    /// # Returns
    ///
    /// The TypeId assigned to the newly registered type
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
    /// # Arguments
    ///
    /// * `name` - The name of the type to look up
    ///
    /// # Returns
    ///
    /// Some(TypeId) if found, None otherwise
    pub fn get_type_by_name(&self, name: &str) -> Option<&TypeId> {
        self.type_names.get(name)
    }

    /// Gets type information for a given TypeId
    ///
    /// # Arguments
    ///
    /// * `id` - The TypeId to look up
    ///
    /// # Returns
    ///
    /// Some(TypeInfo) if found, None otherwise
    pub fn get_type_info(&self, id: &TypeId) -> Option<&TypeInfo> {
        self.types.get(id)
    }

    /// Try to get the primitive type for a given TypeId
    ///
    /// # Arguments
    ///
    /// * `id` - The TypeId to look up
    ///
    /// # Returns
    ///
    /// Some(PrimitiveType) if it's a primitive type, None otherwise
    pub fn get_primitive_type(&self, id: &TypeId) -> Option<PrimitiveType> {
        self.get_type_info(id)
            .and_then(|info| PrimitiveType::from_str(&info.name))
    }

    /// Check if a type is a primitive type
    ///
    /// # Arguments
    ///
    /// * `id` - The TypeId to check
    ///
    /// # Returns
    ///
    /// true if the type is a primitive type, false otherwise
    pub fn is_primitive_type(&self, id: &TypeId) -> bool {
        self.get_primitive_type(id).is_some()
    }

    /// Checks if a value is within the valid range for a given type
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check
    /// * `type_id` - The type to check against
    ///
    /// # Returns
    ///
    /// true if the value is in range, false otherwise
    pub fn check_value_in_range(&self, value: &i64, type_id: &TypeId) -> bool {
        let type_info = match self.get_type_info(type_id) {
            Some(info) => info,
            None => return false,
        };

        match &type_info.kind {
            TypeKind::Integer(int_type) => {
                match (int_type.signed, int_type.bits) {
                    (true, 32) => *value >= i32::MIN as i64 && *value <= i32::MAX as i64,
                    (true, 64) => true, // all i64 fit in i64
                    (false, 32) => *value >= 0 && *value <= u32::MAX as i64,
                    (false, 64) => *value >= 0, // all positive i64 fit in u64
                    _ => false,
                }
            }
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
    /// # Arguments
    ///
    /// * `value` - The value to check
    /// * `type_id` - The type to check against
    ///
    /// # Returns
    ///
    /// true if the value is in range, false otherwise
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
