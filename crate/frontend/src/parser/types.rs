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

/// Extension trait for type parsing functionality
/// 
/// This trait extends the Parser with type parsing methods,
/// providing a clean interface for parsing all type expressions.
pub trait TypeParsing {
    /// Parse a type
    fn parse_type(&mut self) -> Result<TypeId, ParseError>;
    
    /// Parse function type expression
    fn parse_function_type_expression(&mut self) -> Result<Expression, ParseError>;
}

impl<'a> TypeParsing for Parser<'a> {
    /// Parses a type name
    ///
    /// ### Returns
    ///
    /// The type ID for the parsed type or an error
    fn parse_type(&mut self) -> Result<TypeId, ParseError> {
        // Handle function types: fn(param_types) -> return_type
        if self.check(&Tokentype::Fn) {
            self.advance(); // consume 'fn'

            if !self.match_token(&Tokentype::LeftParen) {
                return Err(
                    ParseErrorFactory::expected_opening_paren(self.current_location(), Some("after 'fn'"))
                );
            }

            let mut param_types = Vec::new();
            if !self.check(&Tokentype::RightParen) {
                loop {
                    param_types.push(self.parse_type()?);
                    if !self.match_token(&Tokentype::Comma) {
                        break;
                    }
                }
            }

            if !self.match_token(&Tokentype::RightParen) {
                return Err(ParseErrorFactory::expected_closing_paren(
                    self.current_location(),
                    Some("after function parameters"),
                ));
            }

            if !self.match_token(&Tokentype::Arrow) {
                return Err(ParseErrorFactory::invalid_syntax(
                    self.current_location(),
                    "Expected '->' after function parameters",
                    None,
                ));
            }

            let return_type = self.parse_type()?;

            let function_type_id = self
                .context
                .register_function_type(param_types, return_type);
            return Ok(function_type_id);
        }

        if self.match_token(&Tokentype::LeftParen) {
            if !self.match_token(&Tokentype::RightParen) {
                return Err(ParseErrorFactory::expected_closing_paren(
                    self.current_location(),
                    Some("for unit type"),
                ));
            }
            return Ok(PrimitiveType::Unit.into());
        }

        let (type_name, _position) = if let Some((name, position)) = self.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(self.current_location(), Some("type identifier")));
        };

        if type_name == TYPE_NAME_INT {
            return Err(ParseErrorFactory::unknown_type(
                self.current_location(),
                &format!(
                    "'{TYPE_NAME_INT}' is not a valid type specifier. Use '{TYPE_NAME_I32}', '{TYPE_NAME_I64}', '{TYPE_NAME_U32}', or '{TYPE_NAME_U64}' instead"
                ),
            ));
        } else if type_name == TYPE_NAME_FLOAT {
            return Err(ParseErrorFactory::unknown_type(
                self.current_location(),
                &format!(
                    "'{TYPE_NAME_FLOAT}' is not a valid type specifier. Use '{TYPE_NAME_F32}' or '{TYPE_NAME_F64}' instead"
                ),
            ));
        } else if type_name == TYPE_NAME_UNKNOWN {
            return Err(ParseErrorFactory::unknown_type(
                self.current_location(),
                &format!("'{TYPE_NAME_UNKNOWN}' is not a valid type specifier"),
            ));
        }
        if let Some(symbol) = self.context.lookup_symbol(&type_name) {
            if symbol.kind() == SymbolKind::Type {
                Ok(symbol.type_id)
            } else {
                Err(ParseErrorFactory::unknown_type(
                    self.current_location(),
                    &format!("'{type_name}' is not a type name"),
                ))
            }
        } else {
            Err(ParseErrorFactory::unknown_type(
                self.current_location(),
                &format!("Unknown type: {type_name}"),
            ))
        }
    }

    /// Parses a function type expression: `fn(type1, type2) -> return_type`
    ///
    /// ### Returns
    ///
    /// The parsed function type expression or an error message
    fn parse_function_type_expression(&mut self) -> Result<Expression, ParseError> {
        let fn_token_pos = self.previous().pos;

        if !self.match_token(&Tokentype::LeftParen) {
            return Err(ParseErrorFactory::expected_opening_paren(self.current_location(), Some("'(' after 'fn'")));
        }

        let mut param_types = Vec::new();
        if !self.check(&Tokentype::RightParen) {
            loop {
                param_types.push(self.parse_type()?);
                if !self.match_token(&Tokentype::Comma) {
                    break;
                }
            }
        }

        if !self.match_token(&Tokentype::RightParen) {
            return Err(ParseErrorFactory::expected_closing_paren(
                self.current_location(),
                Some("after function parameters"),
            ));
        }

        if !self.match_token(&Tokentype::Arrow) {
            return Err(ParseErrorFactory::invalid_syntax(
                self.current_location(),
                "Expected '->' after function parameters",
                None,
            ));
        }

        let return_type = self.parse_type()?;

        // Use utility function for cleaner location calculation
        let end_pos = self.previous().pos + self.previous().lexeme.len();
        let location = self.location_from_range(fn_token_pos, end_pos);

        Ok(slang_ir::ExprFactory::function_type_with_location(
            param_types,
            return_type,
            location,
        ))
    }
}
