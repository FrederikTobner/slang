//! AST-specific type inference utilities
//!
//! This module provides type inference for AST nodes, leveraging the
//! general type operations from the slang_types crate.

use crate::ast::{BinaryOperator, Expression, UnaryOperator};
use slang_types::{TypeId, TypeOperations};

/// AST-aware type inference system
pub struct TypeInference;

impl TypeInference {
    /// Infer the result type of a binary operation
    pub fn infer_binary_result(
        left: &Expression,
        op: &BinaryOperator,
        right: &Expression,
    ) -> Option<TypeId> {
        match op {
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide => {
                TypeOperations::promote_arithmetic_types(left.expr_type(), right.expr_type())
            }
            BinaryOperator::GreaterThan
            | BinaryOperator::LessThan
            | BinaryOperator::GreaterThanOrEqual
            | BinaryOperator::LessThanOrEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual => Some(TypeId::bool()),
            BinaryOperator::And | BinaryOperator::Or => Some(TypeId::bool()),
        }
    }

    /// Infer the result type of a unary operation
    #[inline(always)]
    pub fn infer_unary_result(op: &UnaryOperator, operand: &Expression) -> Option<TypeId> {
        match op {
            UnaryOperator::Negate => Some(operand.expr_type()),
            UnaryOperator::Not => Some(TypeId::bool()),
        }
    }

    /// Infer a common type between two expressions
    ///
    /// This is used for conditional expressions where both branches
    /// need to have compatible types.
    #[inline(always)]
    pub fn infer_common_type(left: &Expression, right: &Expression) -> Option<TypeId> {
        let left_type = left.expr_type();
        let right_type = right.expr_type();

        if left_type == right_type {
            Some(left_type)
        } else {
            TypeOperations::promote_arithmetic_types(left_type, right_type)
        }
    }
}
