// Statement parsing module
// Contains logic for parsing all statement types

use super::core::Parser;
use super::error::ParseError;
use crate::token::Tokentype;
use slang_error::ErrorCode;
use slang_ir::Location;
use slang_ir::ast::{
    AssignmentStatement, Expression, FunctionDeclarationStmt, IfStatement, LetStatement, Parameter,
    ReturnStatement, Statement, TypeDefinitionStmt,
};
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

        if !parser.check(&Tokentype::Identifier) {
            return Err(parser.error(
                ErrorCode::ExpectedIdentifier,
                "Expected identifier after 'let'",
            ));
        }

        let token_pos = parser.peek().pos;
        let (line, column) = parser.line_info.get_line_col(token_pos);

        let token = parser.advance();
        let name = token.lexeme.clone();
        let location = Location::new(token_pos, line, column, name.len());
        let mut var_type = PrimitiveType::Unknown.into();

        if parser.match_token(&Tokentype::Colon) {
            var_type = parser.parse_type()?;
        }

        if !parser.match_token(&Tokentype::Equal) {
            return Err(parser.error(
                ErrorCode::ExpectedEquals,
                "Expected '=' after variable name",
            ));
        }

        let expr = parser.expression()?;

        if !parser.match_token(&Tokentype::Semicolon) {
            return Err(parser.error(
                ErrorCode::ExpectedSemicolon,
                "Expected ';' after let statement",
            ));
        }

        Ok(Statement::Let(LetStatement {
            name,
            is_mutable,
            value: expr,
            expr_type: var_type,
            location,
        }))
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
        if !parser.check(&Tokentype::Identifier) {
            return Err(parser.error(
                ErrorCode::ExpectedIdentifier,
                &format!("Expected function name found {}", parser.peek().token_type),
            ));
        }

        let token = parser.advance();
        let name = token.lexeme.clone();
        let token_pos = token.pos;
        let (line, column) = parser.line_info.get_line_col(token_pos);

        if !parser.match_token(&Tokentype::LeftParen) {
            return Err(parser.error(
                ErrorCode::ExpectedOpeningParen,
                &format!(
                    "Expected '(' after function name found {}",
                    parser.peek().token_type
                ),
            ));
        }

        let mut parameters = Vec::new();
        if !parser.check(&Tokentype::RightParen) {
            parameters.push(Self::parse_parameter(parser)?);

            while parser.match_token(&Tokentype::Comma) {
                if parameters.len() >= 255 {
                    return Err(parser.error(
                        ErrorCode::InvalidSyntax,
                        "Cannot have more than 255 parameters",
                    ));
                }
                parameters.push(Self::parse_parameter(parser)?);
            }
        }

        if !parser.match_token(&Tokentype::RightParen) {
            return Err(parser.error(
                ErrorCode::ExpectedClosingParen,
                &format!(
                    "Expected ')' after parameters found {}",
                    parser.peek().token_type
                ),
            ));
        }

        let return_type = if parser.match_token(&Tokentype::Arrow) {
            parser.parse_type()?
        } else {
            PrimitiveType::Unit.into()
        };

        if !parser.match_token(&Tokentype::LeftBrace) {
            return Err(parser.error(
                ErrorCode::ExpectedOpeningBrace,
                "Expected '{' before function body",
            ));
        }

        let body = parser.parse_block_expression()?;

        let end_pos = parser.previous().pos + parser.previous().lexeme.len();
        let location = Location::new(token_pos, line, column, end_pos - token_pos);

        Ok(Statement::FunctionDeclaration(FunctionDeclarationStmt {
            name,
            parameters,
            return_type,
            body,
            location,
        }))
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
        let return_token = parser.previous();
        let token_pos = return_token.pos;
        let (line, column) = parser.line_info.get_line_col(token_pos);

        let value = if !parser.check(&Tokentype::Semicolon) {
            Some(parser.expression()?)
        } else {
            None
        };

        if !parser.match_token(&Tokentype::Semicolon) {
            return Err(parser.error(
                ErrorCode::ExpectedSemicolon,
                "Expected ';' after return statement",
            ));
        }

        let end_pos = parser.previous().pos + parser.previous().lexeme.len();
        let location = Location::new(token_pos, line, column, end_pos - token_pos);

        Ok(Statement::Return(ReturnStatement { value, location }))
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
        if !parser.check(&Tokentype::Identifier) {
            return Err(parser.error(
                ErrorCode::ExpectedIdentifier,
                "Expected struct name after 'struct' keyword",
            ));
        }

        let token = parser.peek();
        let location = parser.source_location_from_token(token);
        let name = parser.advance().lexeme.clone();

        if !parser.match_token(&Tokentype::LeftBrace) {
            return Err(parser.error(
                ErrorCode::ExpectedOpeningBrace,
                "Expected '{' after struct name",
            ));
        }

        let mut fields = Vec::new();

        while !parser.check(&Tokentype::RightBrace) && !parser.is_at_end() {
            if !parser.check(&Tokentype::Identifier) {
                return Err(parser.error(ErrorCode::ExpectedIdentifier, "Expected field name"));
            }

            let field_name = parser.advance().lexeme.clone();

            if !parser.match_token(&Tokentype::Colon) {
                return Err(parser.error(ErrorCode::ExpectedColon, "Expected ':' after field name"));
            }

            let field_type = parser.parse_type()?;
            fields.push((field_name, field_type));

            if !parser.match_token(&Tokentype::Comma) && !parser.check(&Tokentype::RightBrace) {
                return Err(
                    parser.error(ErrorCode::ExpectedComma, "Expected ',' after field or '}'")
                );
            }
        }

        if !parser.match_token(&Tokentype::RightBrace) {
            return Err(parser.error(
                ErrorCode::ExpectedClosingBrace,
                "Expected '}' after struct fields",
            ));
        }

        if !parser.match_token(&Tokentype::Semicolon) {
            return Err(parser.error(
                ErrorCode::ExpectedSemicolon,
                "Expected ';' after struct definition",
            ));
        }

        Ok(Statement::TypeDefinition(TypeDefinitionStmt {
            name,
            fields,
            location,
        }))
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
        if !parser.check(&Tokentype::Identifier) {
            return Err(parser.error(
                ErrorCode::ExpectedIdentifier,
                "Expected identifier for assignment",
            ));
        }

        let token_pos = parser.peek().pos;
        let (line, column) = parser.line_info.get_line_col(token_pos);
        let token = parser.advance();
        let name = token.lexeme.clone();

        if !parser.match_token(&Tokentype::Equal) {
            return Err(parser.error(ErrorCode::ExpectedEquals, "Expected '=' for assignment"));
        }

        let value = parser.expression()?;

        if !parser.match_token(&Tokentype::Semicolon) {
            return Err(parser.error(
                ErrorCode::ExpectedSemicolon,
                "Expected ';' after assignment",
            ));
        }

        let end_pos = parser.previous().pos + parser.previous().lexeme.len();
        let location = Location::new(token_pos, line, column, end_pos - token_pos);

        Ok(Statement::Assignment(AssignmentStatement {
            name,
            value,
            location,
        }))
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
        let (line, column) = parser.line_info.get_line_col(if_token_pos);

        let condition = parser.expression()?;

        if !parser.match_token(&Tokentype::LeftBrace) {
            return Err(parser.error(
                ErrorCode::ExpectedOpeningBrace,
                "Expected '{' after if condition",
            ));
        }

        let then_branch = parser.parse_block_expression()?;

        let else_branch = if parser.match_token(&Tokentype::Else) {
            if !parser.match_token(&Tokentype::LeftBrace) {
                return Err(
                    parser.error(ErrorCode::ExpectedOpeningBrace, "Expected '{' after else")
                );
            }
            Some(parser.parse_block_expression()?)
        } else {
            None
        };

        let end_pos = parser.previous().pos + parser.previous().lexeme.len();
        let location = Location::new(if_token_pos, line, column, end_pos - if_token_pos);

        Ok(Statement::If(IfStatement {
            condition,
            then_branch,
            else_branch,
            location,
        }))
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
                    return Err(parser.error(
                        ErrorCode::ExpectedSemicolon,
                        "Expected ';' after expression",
                    ));
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
        if !parser.check(&Tokentype::Identifier) {
            return Err(parser.error(ErrorCode::ExpectedIdentifier, "Expected parameter name"));
        }

        let token_pos = parser.peek().pos;
        let token = parser.advance();
        let name = token.lexeme.clone();
        let (line, column) = parser.line_info.get_line_col(token_pos);

        if !parser.match_token(&Tokentype::Colon) {
            return Err(parser.error(
                ErrorCode::ExpectedColon,
                "Expected ':' after parameter name",
            ));
        }

        let param_type = parser.parse_type()?;
        let location = Location::new(token_pos, line, column, name.len());

        Ok(Parameter {
            name,
            param_type,
            location,
        })
    }
}
