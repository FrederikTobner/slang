//! Core type identifier and fundamental type system concepts
//!
//! This module defines the basic building blocks of the type system,
//! including the TypeId struct which serves as a unique identifier for all types.

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::primitive::PrimitiveType;

/// A unique identifier for a type in the type system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub usize);

impl Default for TypeId {
    fn default() -> Self {
        TypeId::unknown()
    }
}

impl TypeId {
    /// Creates a new unique type identifier for custom types
    pub fn new() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1000); // above primitive type range
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
        static PRIMITIVE_IDS: LazyLock<HashMap<PrimitiveType, TypeId>> = LazyLock::new(|| {
            let mut map = HashMap::new();

            for primitive in PrimitiveType::iter() {
                map.insert(primitive, TypeId(primitive as usize));
            }
            map
        });

        PRIMITIVE_IDS
            .get(&primitive)
            .cloned()
            .unwrap_or_else(|| panic!("Unknown primitive type: {primitive:?}"))
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

impl From<PrimitiveType> for TypeId {
    fn from(primitive: PrimitiveType) -> Self {
        TypeId::from_primitive(primitive)
    }
}
