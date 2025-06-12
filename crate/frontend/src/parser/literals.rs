// Literal parsing module
// Contains logic for parsing literal values (integers, floats, strings, booleans)

use slang_error::ErrorCode;
use crate::token::Tokentype;
use super::error::ParseError;
use slang_ir::ast::{Expression, LiteralExpr, LiteralValue};
use slang_types::{
    PrimitiveType, TYPE_NAME_F32, TYPE_NAME_F64, TYPE_NAME_I32, TYPE_NAME_I64,
    TYPE_NAME_U32, TYPE_NAME_U64,
};
use super::core::Parser;

/// Literal parser module providing static methods for parsing literal expressions
pub struct LiteralParser;

impl LiteralParser {
    /// Parses an integer literal with optional type suffix
    ///
    /// ### Returns
    ///
    /// The parsed integer literal expression or an error message
    pub fn parse_integer(parser: &mut Parser) -> Result<Expression, ParseError> {
        let token = parser.previous();
        let value_str = token.lexeme.clone();
        let base_value = value_str.parse::<i64>().map_err(|_| {
            parser.error_previous(
                ErrorCode::InvalidNumberLiteral,
                &format!("Invalid integer: {}", value_str),
            )
        })?;
        let location = parser.source_location_from_token(token);

        if parser.check(&Tokentype::Identifier) {
            let type_name = parser.peek().lexeme.clone();

            match type_name.as_str() {
                TYPE_NAME_I32 => {
                    parser.advance();
                    if base_value > i32::MAX as i64 || base_value < i32::MIN as i64 {
                        return Err(parser.error_previous(
                            ErrorCode::ValueOutOfRange,
                            &format!("Value {} is out of range for {}", base_value, TYPE_NAME_I32),
                        ));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::I32(base_value as i32),
                        expr_type: PrimitiveType::I32.into(),
                        location,
                    }));
                }
                TYPE_NAME_I64 => {
                    parser.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::I64(base_value),
                        expr_type: PrimitiveType::I64.into(),
                        location,
                    }));
                }
                TYPE_NAME_U32 => {
                    parser.advance();
                    if base_value < 0 || base_value > u32::MAX as i64 {
                        return Err(parser.error_previous(
                            ErrorCode::ValueOutOfRange,
                            &format!("Value {} is out of range for {}", base_value, TYPE_NAME_U32),
                        ));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::U32(base_value as u32),
                        expr_type: PrimitiveType::U32.into(),
                        location,
                    }));
                }
                TYPE_NAME_U64 => {
                    parser.advance();
                    if base_value < 0 {
                        return Err(parser.error_previous(
                            ErrorCode::ValueOutOfRange,
                            &format!("Value {} is out of range for {}", base_value, TYPE_NAME_U64),
                        ));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::U64(base_value as u64),
                        expr_type: PrimitiveType::U64.into(),
                        location,
                    }));
                }
                TYPE_NAME_F32 => {
                    parser.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F32(base_value as f32),
                        expr_type: PrimitiveType::F32.into(),
                        location,
                    }));
                }
                TYPE_NAME_F64 => {
                    parser.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F64(base_value as f64),
                        expr_type: PrimitiveType::F64.into(),
                        location,
                    }));
                }
                _ => {}
            }
        }

        Ok(Expression::Literal(LiteralExpr {
            value: LiteralValue::UnspecifiedInteger(base_value),
            expr_type: PrimitiveType::UnspecifiedInt.into(),
            location,
        }))
    }

    /// Parses a float literal with optional type suffix
    ///
    /// ### Returns
    ///
    /// The parsed float literal expression or an error message
    pub fn parse_float(parser: &mut Parser) -> Result<Expression, ParseError> {
        let token = parser.previous();
        let value_str = token.lexeme.clone();
        let location = parser.source_location_from_token(token);
        let value = value_str.parse::<f64>().map_err(|_| {
            parser.error_previous(
                ErrorCode::InvalidNumberLiteral,
                &format!("Invalid float: {}", value_str),
            )
        })?;

        if parser.check(&Tokentype::Identifier) {
            let type_name = parser.peek().lexeme.clone();

            match type_name.as_str() {
                TYPE_NAME_F32 => {
                    parser.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F32(value as f32),
                        expr_type: PrimitiveType::F32.into(),
                        location,
                    }));
                }
                TYPE_NAME_F64 => {
                    parser.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F64(value),
                        expr_type: PrimitiveType::F64.into(),
                        location,
                    }));
                }
                _ => {}
            }
        }

        Ok(Expression::Literal(LiteralExpr {
            value: LiteralValue::UnspecifiedFloat(value),
            expr_type: PrimitiveType::UnspecifiedFloat.into(),
            location,
        }))
    }
}
