// Statement parsing module
// Contains logic for parsing all statement types

use super::core::Parser;
use slang_error::{ParseError, ParseErrorFactory};
use crate::token::Tokentype;
use slang_ir::ast::{
    Expression, Parameter, Statement,
};
use slang_ir::StmtFactory;
use slang_types::PrimitiveType;

/// Statement parser that handles all statement types
pub struct StatementParser;

impl StatementParser {
    /// Parses a single statement
    ///
    /// ### Arguments
    ///
    /// * `parser` - Reference to the core parser
    ///
    /// ### Returns
    ///
    /// The parsed statement or an error message
    pub fn parse_statement(parser: &mut Parser) -> Result<Statement, ParseError> {
        if parser.match_token(&Tokentype::Let) {
            Self::parse_let_statement(parser)
        } else if parser.match_token(&Tokentype::Struct) {
            Self::parse_type_definition(parser)
        } else if parser.match_token(&Tokentype::Fn) {
            Self::parse_function_declaration(parser)
        } else if parser.match_token(&Tokentype::Return) {
            Self::parse_return_statement(parser)
        } else if parser.match_token(&Tokentype::If) {
            Self::parse_if_statement(parser)
        } else if parser.check(&Tokentype::Identifier) && parser.check_next(&Tokentype::Equal) {
            Self::parse_assignment_statement(parser)
        } else {
            Self::parse_expression_statement(parser)
        }
    }

    /// Parses a let statement
    ///
    /// ### Arguments
    ///
    /// * `parser` - Reference to the core parser
    ///
    /// ### Returns
    ///
    /// The parsed let statement or an error message
    fn parse_let_statement(parser: &mut Parser) -> Result<Statement, ParseError> {
        let is_mutable = parser.match_token(&Tokentype::Mut);

        let (name, position) = if let Some((name, position)) = parser.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(parser.current_location(), Some("Expected identifier after 'let'")));
        };

        let location = parser.location_from_range(position.pos, position.end_pos());
        let mut var_type = PrimitiveType::Unknown.into();

        if parser.match_token(&Tokentype::Colon) {
            var_type = parser.parse_type()?;
        }

        if !parser.match_token(&Tokentype::Equal) {
            return Err(ParseErrorFactory::expected_equals(parser.current_location()));
        }

        let expr = parser.expression()?;

        if !parser.match_token(&Tokentype::Semicolon) {
            return Err(ParseErrorFactory::expected_semicolon(parser.current_location(), Some("after let statement")));
        }

        // Use factory based on whether we have explicit type annotation
        let stmt = if var_type != PrimitiveType::Unknown.into() {
            // Explicit type annotation - use typed declaration  
            if is_mutable {
                Statement::Let(StmtFactory::let_mut_typed_stmt_with_location(name, var_type, expr, location))
            } else {
                Statement::Let(StmtFactory::let_typed_stmt_with_location(name, var_type, expr, location))
            }
        } else {
            // No explicit type - use type inference with proper location
            if is_mutable {
                Statement::Let(StmtFactory::let_mut_stmt_with_location(name, expr, location))
            } else {
                Statement::Let(StmtFactory::let_var_stmt_with_location(name, expr, location))
            }
        };

        Ok(stmt)
    }

    /// Parses a function declaration statement
    ///
    /// ### Arguments
    ///
    /// * `parser` - Reference to the core parser
    ///
    /// ### Returns
    ///
    /// The parsed function declaration or an error message
    fn parse_function_declaration(parser: &mut Parser) -> Result<Statement, ParseError> {
        let (name, position) = if let Some((name, position)) = parser.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(parser.current_location(), Some(&format!(
                "function name found {}",
                parser.peek().token_type
            ))));
        };

        let name_location = parser.location_from_range(position.pos, position.end_pos());

        if !parser.match_token(&Tokentype::LeftParen) {
            return Err(ParseErrorFactory::expected_opening_paren(parser.current_location(), Some(&format!(
                "after function name found {}",
                parser.peek().token_type
            ))));
        }

        let mut parameters = Vec::new();
        if !parser.check(&Tokentype::RightParen) {
            parameters.push(Self::parse_parameter(parser)?);

            while parser.match_token(&Tokentype::Comma) {
                if parameters.len() >= 255 {
                    return Err(ParseErrorFactory::invalid_syntax(
                        parser.current_location(),
                        "Cannot have more than 255 parameters",
                        None
                    ));
                }
                parameters.push(Self::parse_parameter(parser)?);
            }
        }

        if !parser.match_token(&Tokentype::RightParen) {
            return Err(ParseErrorFactory::expected_closing_paren(parser.current_location(), Some(&format!(
                "after parameters found {}",
                parser.peek().token_type
            ))));
        }

        let return_type = if parser.match_token(&Tokentype::Arrow) {
            parser.parse_type()?
        } else {
            PrimitiveType::Unit.into()
        };

        if !parser.match_token(&Tokentype::LeftBrace) {
            return Err(ParseErrorFactory::expected_opening_brace(parser.current_location(), Some("before function body")));
        }

        let body = parser.parse_block_expression()?;

        Ok(Statement::FunctionDeclaration(StmtFactory::function_stmt_with_location(name, parameters, return_type, body, name_location)))
    }

    /// Parses a return statement
    ///
    /// ### Arguments
    ///
    /// * `parser` - Reference to the core parser
    ///
    /// ### Returns
    ///
    /// The parsed return statement or an error message
    fn parse_return_statement(parser: &mut Parser) -> Result<Statement, ParseError> {
        let return_token_pos = parser.previous().pos;

        let value = if !parser.check(&Tokentype::Semicolon) {
            Some(parser.expression()?)
        } else {
            None
        };

        if !parser.match_token(&Tokentype::Semicolon) {
            return Err(ParseErrorFactory::expected_semicolon(parser.current_location(), Some("after return statement")));
        }

        // Use utility function for cleaner location calculation
        let semicolon_end = parser.previous().pos + parser.previous().lexeme.len();
        let location = parser.location_from_range(return_token_pos, semicolon_end);

        // Use factory based on whether we have a return value
        let stmt = if let Some(expr) = value {
            Statement::Return(StmtFactory::return_value_stmt_with_location(expr, location))
        } else {
            Statement::Return(StmtFactory::return_void_stmt_with_location(location))
        };

        Ok(stmt)
    }

    /// Parses a type definition statement (struct)
    ///
    /// ### Arguments
    ///
    /// * `parser` - Reference to the core parser
    ///
    /// ### Returns
    ///
    /// The parsed type definition or an error message
    fn parse_type_definition(parser: &mut Parser) -> Result<Statement, ParseError> {
        let (name, position) = if let Some((name, position)) = parser.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(parser.current_location(), Some("struct name after 'struct' keyword")));
        };

        let location = parser.location_from_range(position.pos, position.end_pos());

        if !parser.match_token(&Tokentype::LeftBrace) {
            return Err(ParseErrorFactory::expected_opening_brace(parser.current_location(), Some("after struct name")));
        }

        let mut fields = Vec::new();

        while !parser.check(&Tokentype::RightBrace) && !parser.is_at_end() {
            let field_name = if let Some((name, _position)) = parser.match_identifier_token() {
                name.to_string()
            } else {
                return Err(ParseErrorFactory::expected_identifier(parser.current_location(), Some("field name")));
            };

            if !parser.match_token(&Tokentype::Colon) {
                return Err(ParseErrorFactory::expected_colon(parser.current_location()));
            }

            let field_type = parser.parse_type()?;
            fields.push((field_name, field_type));

            if !parser.match_token(&Tokentype::Comma) && !parser.check(&Tokentype::RightBrace) {
                return Err(ParseErrorFactory::expected_comma(parser.current_location(), Some("after field or '}'")));
            }
        }

        if !parser.match_token(&Tokentype::RightBrace) {
            return Err(ParseErrorFactory::expected_closing_brace(parser.current_location(), Some("after struct fields")));
        }

        if !parser.match_token(&Tokentype::Semicolon) {
            return Err(ParseErrorFactory::expected_semicolon(parser.current_location(), Some("after struct definition")));
        }

        Ok(Statement::TypeDefinition(StmtFactory::type_definition_stmt_with_location(name, fields, location)))
    }

    /// Parses an assignment statement
    ///
    /// ### Arguments
    ///
    /// * `parser` - Reference to the core parser
    ///
    /// ### Returns
    ///
    /// The parsed assignment statement or an error message
    fn parse_assignment_statement(parser: &mut Parser) -> Result<Statement, ParseError> {
        let (name, position) = if let Some((name, position)) = parser.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(parser.current_location(), Some("for assignment")));
        };

        let token_pos = position.pos;

        if !parser.match_token(&Tokentype::Equal) {
            return Err(ParseErrorFactory::expected_equals(parser.current_location()));
        }

        let value = parser.expression()?;

        if !parser.match_token(&Tokentype::Semicolon) {
            return Err(ParseErrorFactory::expected_semicolon(parser.current_location(), Some("after assignment")));
        }

        // Calculate proper location span using utility
        let end_pos = parser.previous().pos + parser.previous().lexeme.len();
        let location = parser.location_from_range(token_pos, end_pos);

        Ok(Statement::Assignment(StmtFactory::assign_stmt_with_location(name, value, location)))
    }

    /// Parses an if statement
    ///
    /// ### Arguments
    ///
    /// * `parser` - Reference to the core parser
    ///
    /// ### Returns
    ///
    /// The parsed if statement or an error message
    fn parse_if_statement(parser: &mut Parser) -> Result<Statement, ParseError> {
        let if_token_pos = parser.previous().pos;

        let condition = parser.expression()?;

        if !parser.match_token(&Tokentype::LeftBrace) {
            return Err(ParseErrorFactory::expected_opening_brace(parser.current_location(), Some("after if condition")));
        }

        let then_branch = parser.parse_block_expression()?;

        let else_expr = if parser.match_token(&Tokentype::Else) {
            if !parser.match_token(&Tokentype::LeftBrace) {
                return Err(ParseErrorFactory::expected_opening_brace(parser.current_location(), Some("after else")));
            }
            Some(parser.parse_block_expression()?)
        } else {
            None
        };

        let end_pos = parser.previous().pos + parser.previous().lexeme.len();
        let location = parser.location_from_range(if_token_pos, end_pos);

        Ok(Statement::If(StmtFactory::if_stmt_with_location(condition, then_branch, else_expr, location)))
    }

    /// Parses an expression statement
    ///
    /// ### Arguments
    ///
    /// * `parser` - Reference to the core parser
    ///
    /// ### Returns
    ///
    /// The parsed expression statement or an error message
    fn parse_expression_statement(parser: &mut Parser) -> Result<Statement, ParseError> {
        let expr = parser.expression()?;

        // Block expressions don't need semicolons when used as statements
        match &expr {
            Expression::Block(_) => {
                // No semicolon required for block expressions
            }
            _ => {
                if !parser.match_token(&Tokentype::Semicolon) {
                    return Err(ParseErrorFactory::expected_semicolon(parser.current_location(), Some("after expression")));
                }
            }
        }

        Ok(Statement::Expression(expr))
    }

    /// Parses a function parameter
    ///
    /// ### Arguments
    ///
    /// * `parser` - Reference to the core parser
    ///
    /// ### Returns
    ///
    /// The parsed parameter or an error message
    fn parse_parameter(parser: &mut Parser) -> Result<Parameter, ParseError> {
        let (name, position) = if let Some((name, position)) = parser.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(parser.current_location(), Some("parameter name")));
        };

        if !parser.match_token(&Tokentype::Colon) {
            return Err(ParseErrorFactory::expected_colon(parser.current_location()));
        }

        let param_type = parser.parse_type()?;
        let location = parser.location_from_range(position.pos, position.end_pos());

        Ok(Parameter {
            name,
            param_type,
            location,
        })
    }
}
