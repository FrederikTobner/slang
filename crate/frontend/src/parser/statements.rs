// Statement parsing module
// Contains logic for parsing all statement types

use super::core::Parser;
use crate::token::Tokentype;
use slang_error::{ParseError, ParseErrorFactory};
use slang_ir::StmtFactory;
use slang_ir::ast::{Expression, Parameter, Statement};
use slang_types::PrimitiveType;

/// Extension trait for statement parsing functionality
///
/// This trait extends the Parser with statement parsing methods,
/// providing a clean interface for parsing all statement types.
pub trait StatementParsing {
    /// Parse a single statement
    fn statement(&mut self) -> Result<Statement, ParseError>;

    /// Parse let statement
    fn parse_let_statement(&mut self) -> Result<Statement, ParseError>;

    /// Parse type definition
    fn parse_type_definition(&mut self) -> Result<Statement, ParseError>;

    /// Parse function declaration
    fn parse_function_declaration(&mut self) -> Result<Statement, ParseError>;

    /// Parse return statement
    fn parse_return_statement(&mut self) -> Result<Statement, ParseError>;

    /// Parse if statement
    fn parse_if_statement(&mut self) -> Result<Statement, ParseError>;

    /// Parse assignment statement
    fn parse_assignment_statement(&mut self) -> Result<Statement, ParseError>;

    /// Parse expression statement
    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError>;
}

impl<'a> StatementParsing for Parser<'a> {
    /// Parse a single statement
    ///
    /// ### Returns
    ///
    /// The parsed statement or an error message
    fn statement(&mut self) -> Result<Statement, ParseError> {
        if self.match_token(&Tokentype::Let) {
            self.parse_let_statement()
        } else if self.match_token(&Tokentype::Struct) {
            self.parse_type_definition()
        } else if self.match_token(&Tokentype::Fn) {
            self.parse_function_declaration()
        } else if self.match_token(&Tokentype::Return) {
            self.parse_return_statement()
        } else if self.match_token(&Tokentype::If) {
            self.parse_if_statement()
        } else if self.check(&Tokentype::Identifier) && self.check_next(&Tokentype::Equal) {
            self.parse_assignment_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    /// Parses a let statement
    ///
    /// ### Returns
    ///
    /// The parsed let statement or an error message
    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        let is_mutable = self.match_token(&Tokentype::Mut);

        let (name, position) = if let Some((name, position)) = self.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(
                self.current_location(),
                Some("Expected identifier after 'let'"),
            ));
        };

        let location = self.location_from_range(position.pos, position.end_pos());
        let mut var_type = PrimitiveType::Unknown.into();

        if self.match_token(&Tokentype::Colon) {
            var_type = self.parse_type()?;
        }

        if !self.match_token(&Tokentype::Equal) {
            return Err(ParseErrorFactory::expected_equals(self.current_location()));
        }

        let expr = self.expression()?;

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(ParseErrorFactory::expected_semicolon(
                self.current_location(),
                Some("after let statement"),
            ));
        }

        // Use factory based on whether we have explicit type annotation
        let stmt = if var_type != PrimitiveType::Unknown.into() {
            // Explicit type annotation - use typed declaration
            if is_mutable {
                Statement::Let(StmtFactory::let_mut_typed_stmt_with_location(
                    name, var_type, expr, location,
                ))
            } else {
                Statement::Let(StmtFactory::let_typed_stmt_with_location(
                    name, var_type, expr, location,
                ))
            }
        } else {
            // No explicit type - use type inference with proper location
            if is_mutable {
                Statement::Let(StmtFactory::let_mut_stmt_with_location(
                    name, expr, location,
                ))
            } else {
                Statement::Let(StmtFactory::let_var_stmt_with_location(
                    name, expr, location,
                ))
            }
        };

        Ok(stmt)
    }

    /// Parse type definition
    fn parse_type_definition(&mut self) -> Result<Statement, ParseError> {
        let (name, position) = if let Some((name, position)) = self.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(
                self.current_location(),
                Some("struct name after 'struct' keyword"),
            ));
        };

        let location = self.location_from_range(position.pos, position.end_pos());

        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(ParseErrorFactory::expected_opening_brace(
                self.current_location(),
                Some("after struct name"),
            ));
        }

        let mut fields = Vec::new();

        while !self.check(&Tokentype::RightBrace) && !self.is_at_end() {
            let field_name = if let Some((name, _position)) = self.match_identifier_token() {
                name.to_string()
            } else {
                return Err(ParseErrorFactory::expected_identifier(
                    self.current_location(),
                    Some("field name"),
                ));
            };

            if !self.match_token(&Tokentype::Colon) {
                return Err(ParseErrorFactory::expected_colon(self.current_location()));
            }

            let field_type = self.parse_type()?;
            fields.push((field_name, field_type));

            if !self.match_token(&Tokentype::Comma) && !self.check(&Tokentype::RightBrace) {
                return Err(ParseErrorFactory::expected_comma(
                    self.current_location(),
                    Some("after field or '}'"),
                ));
            }
        }

        if !self.match_token(&Tokentype::RightBrace) {
            return Err(ParseErrorFactory::expected_closing_brace(
                self.current_location(),
                Some("after struct fields"),
            ));
        }

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(ParseErrorFactory::expected_semicolon(
                self.current_location(),
                Some("after struct definition"),
            ));
        }

        Ok(Statement::TypeDefinition(
            StmtFactory::type_definition_stmt_with_location(name, fields, location),
        ))
    }

    /// Parse function declaration
    fn parse_function_declaration(&mut self) -> Result<Statement, ParseError> {
        let (name, position) = if let Some((name, position)) = self.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(
                self.current_location(),
                Some(&format!("function name found {}", self.peek().token_type)),
            ));
        };

        let name_location = self.location_from_range(position.pos, position.end_pos());

        if !self.match_token(&Tokentype::LeftParen) {
            return Err(ParseErrorFactory::expected_opening_paren(
                self.current_location(),
                Some(&format!(
                    "after function name found {}",
                    self.peek().token_type
                )),
            ));
        }

        let mut parameters = Vec::new();
        if !self.check(&Tokentype::RightParen) {
            parameters.push(self.parse_parameter()?);

            while self.match_token(&Tokentype::Comma) {
                if parameters.len() >= 255 {
                    return Err(ParseErrorFactory::invalid_syntax(
                        self.current_location(),
                        "Cannot have more than 255 parameters",
                        None,
                    ));
                }
                parameters.push(self.parse_parameter()?);
            }
        }

        if !self.match_token(&Tokentype::RightParen) {
            return Err(ParseErrorFactory::expected_closing_paren(
                self.current_location(),
                Some(&format!(
                    "after parameters found {}",
                    self.peek().token_type
                )),
            ));
        }

        let return_type = if self.match_token(&Tokentype::Arrow) {
            self.parse_type()?
        } else {
            PrimitiveType::Unit.into()
        };

        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(ParseErrorFactory::expected_opening_brace(
                self.current_location(),
                Some("before function body"),
            ));
        }

        let body = self.parse_block_expression()?;

        Ok(Statement::FunctionDeclaration(
            StmtFactory::function_stmt_with_location(
                name,
                parameters,
                return_type,
                body,
                name_location,
            ),
        ))
    }

    /// Parse return statement
    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let return_token_pos = self.previous().pos;

        let value = if !self.check(&Tokentype::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(ParseErrorFactory::expected_semicolon(
                self.current_location(),
                Some("after return statement"),
            ));
        }

        // Use utility function for cleaner location calculation
        let semicolon_end = self.previous().pos + self.previous().lexeme.len();
        let location = self.location_from_range(return_token_pos, semicolon_end);

        // Use factory based on whether we have a return value
        let stmt = if let Some(expr) = value {
            Statement::Return(StmtFactory::return_value_stmt_with_location(expr, location))
        } else {
            Statement::Return(StmtFactory::return_void_stmt_with_location(location))
        };

        Ok(stmt)
    }

    /// Parse if statement
    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        let if_token_pos = self.previous().pos;

        let condition = self.expression()?;

        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(ParseErrorFactory::expected_opening_brace(
                self.current_location(),
                Some("after if condition"),
            ));
        }

        let then_branch = self.parse_block_expression()?;

        let else_expr = if self.match_token(&Tokentype::Else) {
            if !self.match_token(&Tokentype::LeftBrace) {
                return Err(ParseErrorFactory::expected_opening_brace(
                    self.current_location(),
                    Some("after else"),
                ));
            }
            Some(self.parse_block_expression()?)
        } else {
            None
        };

        let end_pos = self.previous().pos + self.previous().lexeme.len();
        let location = self.location_from_range(if_token_pos, end_pos);

        Ok(Statement::If(StmtFactory::if_stmt_with_location(
            condition,
            then_branch,
            else_expr,
            location,
        )))
    }

    /// Parse assignment statement
    fn parse_assignment_statement(&mut self) -> Result<Statement, ParseError> {
        let (name, position) = if let Some((name, position)) = self.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(
                self.current_location(),
                Some("for assignment"),
            ));
        };

        let token_pos = position.pos;

        if !self.match_token(&Tokentype::Equal) {
            return Err(ParseErrorFactory::expected_equals(self.current_location()));
        }

        let value = self.expression()?;

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(ParseErrorFactory::expected_semicolon(
                self.current_location(),
                Some("after assignment"),
            ));
        }

        // Calculate proper location span using utility
        let end_pos = self.previous().pos + self.previous().lexeme.len();
        let location = self.location_from_range(token_pos, end_pos);

        Ok(Statement::Assignment(
            StmtFactory::assign_stmt_with_location(name, value, location),
        ))
    }

    /// Parse expression statement
    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expression()?;

        // Block expressions don't need semicolons when used as statements
        match &expr {
            Expression::Block(_) => {
                // No semicolon required for block expressions
            }
            _ => {
                if !self.match_token(&Tokentype::Semicolon) {
                    return Err(ParseErrorFactory::expected_semicolon(
                        self.current_location(),
                        Some("after expression"),
                    ));
                }
            }
        }

        Ok(Statement::Expression(expr))
    }
}

impl<'a> Parser<'a> {
    /// Parses a function parameter
    ///
    /// ### Returns
    ///
    /// The parsed parameter or an error message
    fn parse_parameter(&mut self) -> Result<Parameter, ParseError> {
        let (name, position) = if let Some((name, position)) = self.match_identifier_token() {
            (name.to_string(), position)
        } else {
            return Err(ParseErrorFactory::expected_identifier(
                self.current_location(),
                Some("parameter name"),
            ));
        };

        if !self.match_token(&Tokentype::Colon) {
            return Err(ParseErrorFactory::expected_colon(self.current_location()));
        }

        let param_type = self.parse_type()?;
        let location = self.location_from_range(position.pos, position.end_pos());

        Ok(Parameter {
            name,
            param_type,
            location,
        })
    }
}
