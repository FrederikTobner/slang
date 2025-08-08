//! Type inference and promotion utilities
//!
//! This module provides core type system operations that are independent
//! of AST structure, making them reusable across the entire compiler.

use crate::type_id::TypeId;

/// Type promotion system for automatic type inference
///
/// Handles standard numeric type promotion hierarchy following Slang semantics.
pub struct TypePromotion;

impl TypePromotion {
    /// Promote arithmetic types following Slang language rules
    ///
    /// This handles the standard numeric type promotion hierarchy:
    /// - Unspecified literals can be promoted to specific types
    /// - Integer types are promoted to wider integer types
    /// - Float types are promoted to wider float types
    #[inline(always)]
    pub fn promote_arithmetic(left: TypeId, right: TypeId) -> Option<TypeId> {
        TypeOperations::promote_arithmetic_types(left, right)
    }

    /// Check if a type can be promoted to another type
    #[inline(always)]
    pub fn can_promote(from: TypeId, to: TypeId) -> bool {
        if from == TypeId::unspecified_int() && TypeOperations::is_integer_type(to) {
            return true;
        }

        if from == TypeId::unspecified_float() && TypeOperations::is_float_type(to) {
            return true;
        }

        // Exact match is always valid
        from == to
    }
}

/// Core type operations that don't depend on AST structures
pub struct TypeOperations;

impl TypeOperations {
    /// Promote arithmetic types following Slang language rules
    #[inline(always)]
    pub fn promote_arithmetic_types(left: TypeId, right: TypeId) -> Option<TypeId> {
        // Handle unspecified literals - they can be promoted to specific types
        if left == TypeId::unspecified_int() && Self::is_integer_type(right) {
            return Some(right);
        }

        if right == TypeId::unspecified_int() && Self::is_integer_type(left) {
            return Some(left);
        }

        if left == TypeId::unspecified_float() && Self::is_float_type(right) {
            return Some(right);
        }

        if right == TypeId::unspecified_float() && Self::is_float_type(left) {
            return Some(left);
        }

        // Exact type match required for specific types
        if left == right {
            Some(left)
        } else {
            None // No automatic promotion between different specific types
        }
    }

    /// Check if a type is an integer type
    #[inline(always)]
    pub fn is_integer_type(type_id: TypeId) -> bool {
        type_id == TypeId::i32()
            || type_id == TypeId::i64()
            || type_id == TypeId::u32()
            || type_id == TypeId::u64()
            || type_id == TypeId::unspecified_int()
    }

    /// Check if a type is a float type
    #[inline(always)]
    pub fn is_float_type(type_id: TypeId) -> bool {
        type_id == TypeId::f32()
            || type_id == TypeId::f64()
            || type_id == TypeId::unspecified_float()
    }
}

/// Extension trait for TypeId to support type queries
pub trait TypeQueries {
    fn is_numeric(&self) -> bool;
    fn is_integer(&self) -> bool;
    fn is_float(&self) -> bool;
}

impl TypeQueries for TypeId {
    #[inline(always)]
    fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    #[inline(always)]
    fn is_integer(&self) -> bool {
        TypeOperations::is_integer_type(*self)
    }

    #[inline(always)]
    fn is_float(&self) -> bool {
        TypeOperations::is_float_type(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_promotion_unspecified_int() {
        assert_eq!(
            TypeOperations::promote_arithmetic_types(TypeId::unspecified_int(), TypeId::i32()),
            Some(TypeId::i32())
        );
        assert_eq!(
            TypeOperations::promote_arithmetic_types(TypeId::i64(), TypeId::unspecified_int()),
            Some(TypeId::i64())
        );
    }

    #[test]
    fn test_type_promotion_unspecified_float() {
        assert_eq!(
            TypeOperations::promote_arithmetic_types(TypeId::unspecified_float(), TypeId::f32()),
            Some(TypeId::f32())
        );
        assert_eq!(
            TypeOperations::promote_arithmetic_types(TypeId::f64(), TypeId::unspecified_float()),
            Some(TypeId::f64())
        );
    }

    #[test]
    fn test_type_promotion_exact_match() {
        assert_eq!(
            TypeOperations::promote_arithmetic_types(TypeId::i32(), TypeId::i32()),
            Some(TypeId::i32())
        );
        assert_eq!(
            TypeOperations::promote_arithmetic_types(TypeId::f64(), TypeId::f64()),
            Some(TypeId::f64())
        );
    }

    #[test]
    fn test_type_promotion_incompatible() {
        assert_eq!(
            TypeOperations::promote_arithmetic_types(TypeId::i32(), TypeId::f32()),
            None
        );
        assert_eq!(
            TypeOperations::promote_arithmetic_types(TypeId::bool(), TypeId::i32()),
            None
        );
    }

    #[test]
    fn test_type_queries_trait() {
        assert!(TypeId::i32().is_integer());
        assert!(TypeId::f64().is_float());
        assert!(TypeId::i32().is_numeric());
        assert!(TypeId::f32().is_numeric());
        assert!(!TypeId::bool().is_numeric());
        assert!(!TypeId::string().is_integer());
    }

    #[test]
    fn test_type_can_promote() {
        assert!(TypePromotion::can_promote(
            TypeId::unspecified_int(),
            TypeId::i32()
        ));
        assert!(TypePromotion::can_promote(
            TypeId::unspecified_float(),
            TypeId::f64()
        ));
        assert!(TypePromotion::can_promote(TypeId::i32(), TypeId::i32())); // exact match
        assert!(!TypePromotion::can_promote(TypeId::i32(), TypeId::f32()));
    }

    #[test]
    fn test_integer_type_detection() {
        assert!(TypeOperations::is_integer_type(TypeId::i32()));
        assert!(TypeOperations::is_integer_type(TypeId::i64()));
        assert!(TypeOperations::is_integer_type(TypeId::u32()));
        assert!(TypeOperations::is_integer_type(TypeId::u64()));
        assert!(TypeOperations::is_integer_type(TypeId::unspecified_int()));
        assert!(!TypeOperations::is_integer_type(TypeId::f32()));
        assert!(!TypeOperations::is_integer_type(TypeId::bool()));
    }

    #[test]
    fn test_float_type_detection() {
        assert!(TypeOperations::is_float_type(TypeId::f32()));
        assert!(TypeOperations::is_float_type(TypeId::f64()));
        assert!(TypeOperations::is_float_type(TypeId::unspecified_float()));
        assert!(!TypeOperations::is_float_type(TypeId::i32()));
        assert!(!TypeOperations::is_float_type(TypeId::bool()));
    }
}
