// Type parsing module
// Contains logic for parsing type expressions and type resolution

use super::core::Parser;
use slang_error::{ParseError, ParseErrorFactory};
use crate::token::Tokentype;
use slang_ir::ast::Expression;
use slang_shared::SymbolKind;
use slang_types::{
    PrimitiveType, TYPE_NAME_F32, TYPE_NAME_F64, TYPE_NAME_FLOAT, TYPE_NAME_I32, TYPE_NAME_I64,
    TYPE_NAME_INT, TYPE_NAME_U32, TYPE_NAME_U64, TYPE_NAME_UNKNOWN, TypeId,
};

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

            if !parser.match_token(&Tokentype::LeftParen) {
                return Err(
                    ParseErrorFactory::expected_opening_paren(parser.current_location(), Some("after 'fn'"))
                );
            }

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
                return Err(ParseErrorFactory::expected_closing_paren(
                    parser.current_location(),
                    Some("after function parameters"),
                ));
            }

            if !parser.match_token(&Tokentype::Arrow) {
                return Err(ParseErrorFactory::invalid_syntax(
                    parser.current_location(),
                    "Expected '->' after function parameters",
                    None,
                ));
            }

            let return_type = Self::parse_type(parser)?;

            let function_type_id = parser
                .context
                .register_function_type(param_types, return_type);
            return Ok(function_type_id);
        }

        if parser.match_token(&Tokentype::LeftParen) {
            if !parser.match_token(&Tokentype::RightParen) {
                return Err(ParseErrorFactory::expected_closing_paren(
                    parser.current_location(),
                    Some("for unit type"),
                ));
            }
            return Ok(PrimitiveType::Unit.into());
        }

        let (type_name, _position) = if let Some((name, position)) = parser.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(parser.current_location(), Some("type identifier")));
        };

        if type_name == TYPE_NAME_INT {
            return Err(ParseErrorFactory::unknown_type(
                parser.current_location(),
                &format!(
                    "'{TYPE_NAME_INT}' is not a valid type specifier. Use '{TYPE_NAME_I32}', '{TYPE_NAME_I64}', '{TYPE_NAME_U32}', or '{TYPE_NAME_U64}' instead"
                ),
            ));
        } else if type_name == TYPE_NAME_FLOAT {
            return Err(ParseErrorFactory::unknown_type(
                parser.current_location(),
                &format!(
                    "'{TYPE_NAME_FLOAT}' is not a valid type specifier. Use '{TYPE_NAME_F32}' or '{TYPE_NAME_F64}' instead"
                ),
            ));
        } else if type_name == TYPE_NAME_UNKNOWN {
            return Err(ParseErrorFactory::unknown_type(
                parser.current_location(),
                &format!("'{TYPE_NAME_UNKNOWN}' is not a valid type specifier"),
            ));
        }
        if let Some(symbol) = parser.context.lookup_symbol(&type_name) {
            if symbol.kind() == SymbolKind::Type {
                Ok(symbol.type_id)
            } else {
                Err(ParseErrorFactory::unknown_type(
                    parser.current_location(),
                    &format!("'{type_name}' is not a type name"),
                ))
            }
        } else {
            Err(ParseErrorFactory::unknown_type(
                parser.current_location(),
                &format!("Unknown type: {type_name}"),
            ))
        }
    }

    /// Parses a function type expression: `fn(type1, type2) -> return_type`
    ///
    /// ### Returns
    ///
    /// The parsed function type expression or an error message
    pub fn parse_function_type_expression(parser: &mut Parser) -> Result<Expression, ParseError> {
        let fn_token_pos = parser.previous().pos;

        if !parser.match_token(&Tokentype::LeftParen) {
            return Err(ParseErrorFactory::expected_opening_paren(parser.current_location(), Some("'(' after 'fn'")));
        }

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
            return Err(ParseErrorFactory::expected_closing_paren(
                parser.current_location(),
                Some("after function parameters"),
            ));
        }

        if !parser.match_token(&Tokentype::Arrow) {
            return Err(ParseErrorFactory::invalid_syntax(
                parser.current_location(),
                "Expected '->' after function parameters",
                None,
            ));
        }

        let return_type = Self::parse_type(parser)?;

        // Use utility function for cleaner location calculation
        let end_pos = parser.previous().pos + parser.previous().lexeme.len();
        let location = parser.location_from_range(fn_token_pos, end_pos);

        Ok(slang_ir::ExprFactory::function_type_with_location(
            param_types,
            return_type,
            location,
        ))
    }
}
