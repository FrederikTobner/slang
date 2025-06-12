// Type parsing module
// Contains logic for parsing type expressions and type resolution

use slang_error::ErrorCode;
use crate::token::Tokentype;
use super::error::ParseError;
use slang_ir::ast::{Expression, FunctionTypeExpr};
use slang_shared::SymbolKind;
use slang_types::{
    PrimitiveType, TYPE_NAME_F32, TYPE_NAME_F64, TYPE_NAME_FLOAT, TYPE_NAME_I32, TYPE_NAME_I64,
    TYPE_NAME_INT, TYPE_NAME_U32, TYPE_NAME_U64, TYPE_NAME_UNKNOWN, TypeId,
};
use super::core::Parser;

/// Type parser module providing static methods for parsing type expressions
pub struct TypeParser;

impl TypeParser {
    /// Parses a type name
    ///
    /// ### Returns
    ///
    /// The type ID for the parsed type or an error
    pub fn parse_type(parser: &mut Parser) -> Result<TypeId, ParseError> {
        // Handle function types: fn(param_types) -> return_type
        if parser.check(&Tokentype::Fn) {
            parser.advance(); // consume 'fn'

            // Expect '('
            if !parser.match_token(&Tokentype::LeftParen) {
                return Err(parser.error(
                    ErrorCode::ExpectedOpeningParen,
                    "Expected '(' after 'fn'",
                ));
            }

            // Parse parameter types
            let mut param_types = Vec::new();
            if !parser.check(&Tokentype::RightParen) {
                loop {
                    param_types.push(Self::parse_type(parser)?);
                    if !parser.match_token(&Tokentype::Comma) {
                        break;
                    }
                }
            }

            // Expect ')'
            if !parser.match_token(&Tokentype::RightParen) {
                return Err(parser.error(
                    ErrorCode::ExpectedClosingParen,
                    "Expected ')' after function parameters",
                ));
            }

            // Expect '->'
            if !parser.match_token(&Tokentype::Arrow) {
                return Err(parser.error(
                    ErrorCode::InvalidSyntax,
                    "Expected '->' after function parameters",
                ));
            }

            // Parse return type
            let return_type = Self::parse_type(parser)?;

            // Register the function type and return its type ID
            let function_type_id = parser.context.register_function_type(param_types, return_type);
            return Ok(function_type_id);
        }

        if parser.check(&Tokentype::LeftParen) {
            parser.advance(); 
            if !parser.match_token(&Tokentype::RightParen) {
                return Err(parser.error(
                    ErrorCode::ExpectedClosingParen,
                    "Expected ')' for unit type",
                ));
            }
            return Ok(PrimitiveType::Unit.into());
        }

        if !parser.check(&Tokentype::Identifier) {
            return Err(parser.error(ErrorCode::ExpectedIdentifier, "Expected type identifier"));
        }

        let type_name_token = parser.advance();
        let type_name = type_name_token.lexeme.clone();

        if type_name == TYPE_NAME_INT {
            return Err(parser.error(
                ErrorCode::UnknownType,
                &format!(
                    "'{}' is not a valid type specifier. Use '{}', '{}', '{}', or '{}' instead",
                    TYPE_NAME_INT, TYPE_NAME_I32, TYPE_NAME_I64, TYPE_NAME_U32, TYPE_NAME_U64
                ),
            ));
        } else if type_name == TYPE_NAME_FLOAT {
            return Err(parser.error(
                ErrorCode::UnknownType,
                &format!(
                    "'{}' is not a valid type specifier. Use '{}' or '{}' instead",
                    TYPE_NAME_FLOAT, TYPE_NAME_F32, TYPE_NAME_F64
                ),
            ));
        } else if type_name == TYPE_NAME_UNKNOWN {
            return Err(parser.error_previous(
                ErrorCode::UnknownType,
                &format!("'{}' is not a valid type specifier", TYPE_NAME_UNKNOWN),
            ));
        }
        if let Some(symbol) = parser.context.lookup_symbol(&type_name) {
            if symbol.kind() == SymbolKind::Type {
                Ok(symbol.type_id.clone())
            } else {
                Err(parser.error_previous(
                    ErrorCode::UnknownType,
                    &format!("'{}' is not a type name", type_name),
                ))
            }
        } else {
            Err(parser.error_previous(
                ErrorCode::UnknownType,
                &format!("Unknown type: {}", type_name),
            ))
        }
    }

    /// Parses a function type expression: `fn(type1, type2) -> return_type`
    ///
    /// ### Returns
    ///
    /// The parsed function type expression or an error message
    pub fn parse_function_type_expression(parser: &mut Parser) -> Result<Expression, ParseError> {
        // Extract position information upfront to avoid borrowing issues
        let fn_token_pos = parser.previous().pos;
        let (start_line, start_column) = parser.line_info.get_line_col(fn_token_pos);

        // Expect '('
        if !parser.match_token(&Tokentype::LeftParen) {
            return Err(parser.error(
                ErrorCode::ExpectedOpeningParen,
                "Expected '(' after 'fn'",
            ));
        }

        // Parse parameter types
        let mut param_types = Vec::new();
        if !parser.check(&Tokentype::RightParen) {
            loop {
                param_types.push(Self::parse_type(parser)?);
                if !parser.match_token(&Tokentype::Comma) {
                    break;
                }
            }
        }

        if !parser.match_token(&Tokentype::RightParen) {
            return Err(parser.error(
                ErrorCode::ExpectedClosingParen,
                "Expected ')' after function parameters",
            ));
        }

        if !parser.match_token(&Tokentype::Arrow) {
            return Err(parser.error(
                ErrorCode::InvalidSyntax,
                "Expected '->' after function parameters",
            ));
        }

        let return_type = Self::parse_type(parser)?;

        let end_token_pos = parser.previous().pos;
        let end_token_lexeme_len = parser.previous().lexeme.len();
        let end_pos = end_token_pos + end_token_lexeme_len;
        let location = slang_ir::location::Location::new(
            fn_token_pos,
            start_line,
            start_column,
            end_pos - fn_token_pos,
        );

        // Will be determined by the semantic analyzer
        let expr_type = PrimitiveType::Unknown.into();

        Ok(Expression::FunctionType(FunctionTypeExpr {
            param_types,
            return_type,
            expr_type,
            location,
        }))
    }
}
