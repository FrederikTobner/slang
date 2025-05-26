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
