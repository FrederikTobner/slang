//! Primitive type definitions and constants
//!
//! This module defines all primitive types supported by the language,
//! including integers, floats, strings, booleans, and special types.

use super::kind::TypeKind;
use slang_derive::{IterableEnum, NamedEnum, NumericEnum};

// Type name constants for convenient access
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
        use super::kind::{FloatType, IntegerType};

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
