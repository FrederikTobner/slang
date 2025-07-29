//! Supporting traits for AST factory system
//!
//! This module provides trait implementations for converting Rust values
//! into AST literal values with proper type inference.

use crate::ast::LiteralValue;
use slang_types::types::TypeId;

/// Trait for converting Rust values into AST literal values
pub trait IntoLiteralValue {
    fn into_literal_value(self) -> LiteralValue;
}

// Implementations for common Rust types
impl IntoLiteralValue for i32 {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        LiteralValue::I32(self)
    }
}

impl IntoLiteralValue for i64 {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        LiteralValue::I64(self)
    }
}

impl IntoLiteralValue for u32 {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        LiteralValue::U32(self)
    }
}

impl IntoLiteralValue for u64 {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        LiteralValue::U64(self)
    }
}

impl IntoLiteralValue for f32 {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        LiteralValue::F32(self)
    }
}

impl IntoLiteralValue for f64 {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        LiteralValue::F64(self)
    }
}

impl IntoLiteralValue for bool {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        LiteralValue::Boolean(self)
    }
}

impl IntoLiteralValue for &str {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        LiteralValue::String(self.to_string())
    }
}

impl IntoLiteralValue for String {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        LiteralValue::String(self)
    }
}

impl IntoLiteralValue for () {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        LiteralValue::Unit
    }
}

// Support for direct LiteralValue (for special cases like UnspecifiedInteger)
impl IntoLiteralValue for LiteralValue {
    #[inline(always)]
    fn into_literal_value(self) -> LiteralValue {
        self
    }
}

/// Extension trait for automatic type inference from literal values
/// 
/// # Design Principles Applied:
/// - **Single Responsibility**: Only handles type inference
pub trait LiteralTypeInference {
    fn infer_type(&self) -> TypeId;
}

impl LiteralTypeInference for LiteralValue {
    #[inline(always)]
    fn infer_type(&self) -> TypeId {
        match self {
            LiteralValue::I32(_) => TypeId::i32(),
            LiteralValue::I64(_) => TypeId::i64(),
            LiteralValue::U32(_) => TypeId::u32(),
            LiteralValue::U64(_) => TypeId::u64(),
            LiteralValue::F32(_) => TypeId::f32(),
            LiteralValue::F64(_) => TypeId::f64(),
            LiteralValue::Boolean(_) => TypeId::bool(),
            LiteralValue::String(_) => TypeId::string(),
            LiteralValue::Unit => TypeId::unit(),
            LiteralValue::UnspecifiedInteger(_) => TypeId::unspecified_int(),
            LiteralValue::UnspecifiedFloat(_) => TypeId::unspecified_float(),
        }
    }
}
