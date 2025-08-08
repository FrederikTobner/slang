// Literal parsing module
// Contains logic for parsing literal values (integers, floats, strings, booleans)

use super::core::Parser;
use slang_error::{ParseError, ParseErrorFactory};
use slang_ir::ast::Expression;
use slang_ir::ExprFactory; // Import factory system
use slang_types::{
    TYPE_NAME_F32, TYPE_NAME_F64, TYPE_NAME_I32, TYPE_NAME_I64, TYPE_NAME_U32,
    TYPE_NAME_U64,
};

/// Extension trait for literal parsing functionality
/// 
/// This trait extends the Parser with literal parsing methods,
/// providing type-safe parsing of different literal types.
pub trait LiteralParsing {
    /// Parse an integer literal with optional type suffix
    fn parse_integer(&mut self) -> Result<Expression, ParseError>;
    
    /// Parse a float literal with optional type suffix
    fn parse_float(&mut self) -> Result<Expression, ParseError>;
}

impl<'a> LiteralParsing for Parser<'a> {
    fn parse_integer(&mut self) -> Result<Expression, ParseError> {
        let token = self.previous();
        let value_str = token.lexeme.clone();
        let base_value = value_str.parse::<i64>().map_err(|_| {
            ParseErrorFactory::invalid_number_literal(
                self.current_location(),
                &value_str,
                &format!("Invalid integer: {value_str}"),
            )
        })?;
        let location = self.source_location_from_token(token);

        // Check if the token has a suffix
        if let Some(ref suffix) = token.suffix {
            match suffix.as_str() {
                TYPE_NAME_I32 => {
                    if base_value > i32::MAX as i64 || base_value < i32::MIN as i64 {
                        return Err(ParseErrorFactory::value_out_of_range(
                            self.current_location(),
                            &base_value.to_string(),
                            &format!("Value {base_value} is out of range for {TYPE_NAME_I32}"),
                        ));
                    }
                    return Ok(Expression::Literal(ExprFactory::literal_expr_with_location(
                        base_value as i32,
                        location
                    )));
                }
                TYPE_NAME_I64 => {
                    return Ok(Expression::Literal(ExprFactory::literal_expr_with_location(
                        base_value,
                        location
                    )));
                }
                TYPE_NAME_U32 => {
                    if base_value < 0 || base_value > u32::MAX as i64 {
                        return Err(ParseErrorFactory::value_out_of_range(
                            self.current_location(),
                            &base_value.to_string(),
                            &format!("Value {base_value} is out of range for {TYPE_NAME_U32}"),
                        ));
                    }
                    return Ok(Expression::Literal(ExprFactory::literal_expr_with_location(
                        base_value as u32,
                        location
                    )));
                }
                TYPE_NAME_U64 => {
                    if base_value < 0 {
                        return Err(ParseErrorFactory::value_out_of_range(
                            self.current_location(),
                            &base_value.to_string(),
                            &format!("Value {base_value} is out of range for {TYPE_NAME_U64}"),
                        ));
                    }
                    return Ok(Expression::Literal(ExprFactory::literal_expr_with_location(
                        base_value as u64,
                        location
                    )));
                }
                _ => {
                    return Err(ParseErrorFactory::unknown_type(
                        self.current_location(),
                        &format!("Unknown integer type suffix: {suffix}"),
                    ));
                }
            }
        }

        // No suffix - create unspecified integer
        Ok(Expression::Literal(ExprFactory::literal_expr_with_location(
            slang_ir::ast::LiteralValue::UnspecifiedInteger(base_value),
            location
        )))
    }

    fn parse_float(&mut self) -> Result<Expression, ParseError> {
        let token = self.previous();
        let value_str = token.lexeme.clone();
        let base_value = value_str.parse::<f64>().map_err(|_| {
            ParseErrorFactory::invalid_number_literal(
                self.current_location(),
                &value_str,
                &format!("Invalid float: {value_str}"),
            )
        })?;
        let location = self.source_location_from_token(token);

        // Check if the token has a suffix
        if let Some(ref suffix) = token.suffix {
            match suffix.as_str() {
                TYPE_NAME_F32 => {
                    if base_value > f32::MAX as f64 || base_value < f32::MIN as f64 {
                        return Err(ParseErrorFactory::value_out_of_range(
                            self.current_location(),
                            &base_value.to_string(),
                            &format!("Value {base_value} is out of range for {TYPE_NAME_F32}"),
                        ));
                    }
                    return Ok(Expression::Literal(ExprFactory::literal_expr_with_location(
                        base_value as f32,
                        location
                    )));
                }
                TYPE_NAME_F64 => {
                    return Ok(Expression::Literal(ExprFactory::literal_expr_with_location(
                        base_value,
                        location
                    )));
                }
                _ => {
                    return Err(ParseErrorFactory::unknown_type(
                        self.current_location(),
                        &format!("Unknown float type suffix: {suffix}"),
                    ));
                }
            }
        }

        // No suffix - create unspecified float
        Ok(Expression::Literal(ExprFactory::literal_expr_with_location(
            slang_ir::ast::LiteralValue::UnspecifiedFloat(base_value),
            location
        )))
    }
}
