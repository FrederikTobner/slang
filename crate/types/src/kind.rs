//! Type kind definitions and specific type structures
//!
//! This module defines the different kinds of types supported by the language,
//! including compound types like structs and functions.

use crate::type_id::TypeId;

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
        FunctionType {
            param_types,
            return_type,
        }
    }
}
