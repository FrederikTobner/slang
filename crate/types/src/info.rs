//! Type information container and related utilities
//!
//! This module defines the TypeInfo struct which combines a type's identifier,
//! name, and kind information into a single container.

use crate::kind::TypeKind;
use crate::type_id::TypeId;

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
