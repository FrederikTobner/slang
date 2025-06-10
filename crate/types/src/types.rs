use slang_derive::{IterableEnum, NamedEnum, NumericEnum};

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
pub const TYPE_NAME_UNIT: &str = PrimitiveType::Unit.name();
pub const TYPE_NAME_UNKNOWN: &str = PrimitiveType::Unknown.name();

/// Represents all primitive types in the language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, NamedEnum, IterableEnum, NumericEnum)]
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
    /// Unit type (similar to Rust's ())
    #[name = "()"]
    Unit,
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
            PrimitiveType::Unit => 0,
            _ => 0,
        }
    }

    /// Get the TypeKind for this primitive type
    ///
    /// This method defines the actual type characteristics for each primitive type,
    /// separating type definition from type registration logic.
    pub fn to_type_kind(&self) -> TypeKind {
        match self {
            PrimitiveType::I32 => TypeKind::Integer(IntegerType {
                signed: true,
                bits: 32,
                is_unspecified: false,
            }),
            PrimitiveType::I64 => TypeKind::Integer(IntegerType {
                signed: true,
                bits: 64,
                is_unspecified: false,
            }),
            PrimitiveType::U32 => TypeKind::Integer(IntegerType {
                signed: false,
                bits: 32,
                is_unspecified: false,
            }),
            PrimitiveType::U64 => TypeKind::Integer(IntegerType {
                signed: false,
                bits: 64,
                is_unspecified: false,
            }),
            PrimitiveType::UnspecifiedInt => TypeKind::Integer(IntegerType {
                signed: true,
                bits: 0,
                is_unspecified: true,
            }),
            PrimitiveType::F32 => TypeKind::Float(FloatType {
                bits: 32,
                is_unspecified: false,
            }),
            PrimitiveType::F64 => TypeKind::Float(FloatType {
                bits: 64,
                is_unspecified: false,
            }),
            PrimitiveType::UnspecifiedFloat => TypeKind::Float(FloatType {
                bits: 0,
                is_unspecified: true,
            }),
            PrimitiveType::String => TypeKind::String,
            PrimitiveType::Bool => TypeKind::Boolean,
            PrimitiveType::Unit => TypeKind::Unit,
            PrimitiveType::Unknown => TypeKind::Unknown,
        }
    }
}

impl From<PrimitiveType> for usize {
    fn from(primitive: PrimitiveType) -> usize {
        primitive as usize    
    }
}

impl From<PrimitiveType> for TypeId {
    fn from(primitive: PrimitiveType) -> Self {
        TypeId::from_primitive(primitive)
    }
}

/// A unique identifier for a type in the type system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeId(pub usize);

impl Default for TypeId {
    fn default() -> Self {
        TypeId::unknown()
    }
}

impl TypeId {
    /// Creates a new unique type identifier for custom types
    pub fn new() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicUsize =
            std::sync::atomic::AtomicUsize::new(1000); // above primitive type range
        TypeId(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }

    /// Creates a TypeId for a primitive type - PREFERRED METHOD
    /// 
    /// This ensures consistent TypeId assignment for primitive types
    /// and is more robust than direct casting.
    /// 
    /// ### Arguments
    /// * `primitive` - The primitive type to create a TypeId for
    /// 
    /// ### Returns
    /// A TypeId that is guaranteed to be unique and consistent for the primitive type
    pub fn from_primitive(primitive: PrimitiveType) -> Self {
        use std::sync::LazyLock;
        use std::collections::HashMap;
        
        static PRIMITIVE_IDS: LazyLock<HashMap<PrimitiveType, TypeId>> = LazyLock::new(|| {
            let mut map = HashMap::new();

            for primitive in PrimitiveType::iter() {
                map.insert(primitive, TypeId(primitive as usize));
            }
            map
        });
        
        PRIMITIVE_IDS.get(&primitive)
            .cloned()
            .unwrap_or_else(|| panic!("Unknown primitive type: {:?}", primitive))
    }

    /// Returns the TypeId for bool type
    #[inline]
    pub fn bool() -> Self {
        Self::from_primitive(PrimitiveType::Bool)
    }

    /// Returns the TypeId for i32 type
    #[inline]
    pub fn i32() -> Self {
        Self::from_primitive(PrimitiveType::I32)
    }

    /// Returns the TypeId for i64 type
    #[inline]
    pub fn i64() -> Self {
        Self::from_primitive(PrimitiveType::I64)
    }

    /// Returns the TypeId for u32 type
    #[inline]
    pub fn u32() -> Self {
        Self::from_primitive(PrimitiveType::U32)
    }

    /// Returns the TypeId for u64 type
    #[inline]
    pub fn u64() -> Self {
        Self::from_primitive(PrimitiveType::U64)
    }

    /// Returns the TypeId for f32 type
    #[inline]
    pub fn f32() -> Self {
        Self::from_primitive(PrimitiveType::F32)
    }

    /// Returns the TypeId for f64 type
    #[inline]
    pub fn f64() -> Self {
        Self::from_primitive(PrimitiveType::F64)
    }

    /// Returns the TypeId for string type
    #[inline]
    pub fn string() -> Self {
        Self::from_primitive(PrimitiveType::String)
    }

    /// Returns the TypeId for unit type
    #[inline]
    pub fn unit() -> Self {
        Self::from_primitive(PrimitiveType::Unit)
    }

    /// Returns the TypeId for unspecified integer type
    #[inline]
    pub fn unspecified_int() -> Self {
        Self::from_primitive(PrimitiveType::UnspecifiedInt)
    }

    /// Returns the TypeId for unspecified float type
    #[inline]
    pub fn unspecified_float() -> Self {
        Self::from_primitive(PrimitiveType::UnspecifiedFloat)
    }

    /// Returns the TypeId for unknown type
    #[inline]
    pub fn unknown() -> Self {
        Self::from_primitive(PrimitiveType::Unknown)
    }
}

/// Represents the different kinds of types in the language
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeKind {
    /// Integer types (signed/unsigned, different bit widths)
    Integer(IntegerType),
    /// Floating point types
    Float(FloatType),
    /// String type
    String,
    /// Boolean type
    Boolean,
    /// Unit type (similar to Rust's ())
    Unit,
    /// Struct type with fields
    Struct(StructType),
    /// Function type with parameters and return type
    Function(FunctionType),
    /// Unknown or not yet determined type
    Unknown,
}

impl TypeKind {
    /// Returns the function type if this is a function, None otherwise
    pub fn as_function(&self) -> Option<&FunctionType> {
        match self {
            TypeKind::Function(func_type) => Some(func_type),
            _ => None,
        }
    }
}

/// Represents an integer type with its properties
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntegerType {
    /// Whether the integer is signed or unsigned
    pub signed: bool,
    /// The number of bits (e.g., 32 for i32)
    pub bits: u8,
    /// Whether this is an unspecified integer (used for literals without explicit type)
    pub is_unspecified: bool,
}

/// Represents a floating point type with its properties
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FloatType {
    /// The number of bits (e.g., 64 for f64)
    pub bits: u8,
    /// Whether this is an unspecified float (used for literals without explicit type)
    pub is_unspecified: bool,
}

/// Represents a struct type with its fields
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// Represents a function type with its parameters and return type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    /// Parameter types of the function
    pub param_types: Vec<TypeId>,
    /// Return type of the function
    pub return_type: TypeId,
}

impl FunctionType {
    /// Creates a new FunctionType.
    pub fn new(param_types: Vec<TypeId>, return_type: TypeId) -> Self {
        FunctionType { param_types, return_type }
    }
}

/// Contains all information about a specific type
#[derive(Debug)]
pub struct TypeInfo {
    /// Unique identifier for this type
    pub id: TypeId,
    /// Name of the type
    pub name: String,
    /// The kind of type (integer, float, string, etc.)
    pub kind: TypeKind,
}
