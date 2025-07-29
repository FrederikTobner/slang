// Utilities module
// Contains helper functions and common parsing patterns

use super::core::Parser;
use super::error::ParseError;
use crate::token::Tokentype;
use slang_error::ErrorCode;
use slang_ir::ast::{BlockExpr, Expression};
use slang_ir::{ExprFactory, StmtFactory}; // Import factory system

/// Utilities parser module providing static methods for common parsing patterns
pub struct UtilitiesParser;

impl UtilitiesParser {
    /// Finishes parsing a function call after the name and '('
    ///
    /// #### Arguments
    ///
    /// * `parser` - The parser instance
    /// * `name` - The name of the function being called
    /// * `name_location` - The location of the function name token
    ///
    /// ### Returns
    ///
    /// The parsed function call expression or an error message
    pub fn finish_call(parser: &mut Parser, name: String, name_location: slang_ir::location::Location) -> Result<Expression, ParseError> {
        let mut arguments = Vec::new();

        if !parser.check(&Tokentype::RightParen) {
            arguments.push(parser.expression()?);

            while parser.match_token(&Tokentype::Comma) {
                if arguments.len() >= 255 {
                    return Err(parser.error(
                        ErrorCode::InvalidSyntax,
                        "Cannot have more than 255 arguments",
                    ));
                }
                arguments.push(parser.expression()?);
            }
        }

        if !parser.match_token(&Tokentype::RightParen) {
            return Err(parser.error(
                ErrorCode::ExpectedClosingParen,
                "Expected ')' after function arguments",
            ));
        }

        // Create function call using factory with original name location
        // The factory will handle extending the location to include arguments
        Ok(Expression::Call(slang_ir::ExprFactory::call_expr_with_location(
            name,
            arguments,
            name_location,
        )))
    }

    /// Parses a conditional expression (if/else expression)
    ///
    /// ### Arguments
    ///
    /// * `parser` - The parser instance
    ///
    /// ### Returns
    ///
    /// The parsed conditional expression or an error message
    pub fn conditional_expression(parser: &mut Parser) -> Result<Expression, ParseError> {
        let condition = parser.expression()?;

        if !parser.match_token(&Tokentype::LeftBrace) {
            return Err(parser.error(
                ErrorCode::ExpectedOpeningBrace,
                "Expected '{' after if condition",
            ));
        }

        let then_branch = Self::parse_block_expression(parser)?;

        if !parser.match_token(&Tokentype::Else) {
            return Err(parser.error(
                ErrorCode::ExpectedElse,
                "Expected 'else' after if expression",
            ));
        }

        if !parser.match_token(&Tokentype::LeftBrace) {
            return Err(parser.error(ErrorCode::ExpectedOpeningBrace, "Expected '{' after else"));
        }

        let else_branch = Self::parse_block_expression(parser)?;

        // Use factory for automatic location calculation from operands
        Ok(ExprFactory::conditional(
            condition,
            ExprFactory::block_from_expr(then_branch),
            ExprFactory::block_from_expr(else_branch),
        ))
    }

    /// Parses a block expression - a sequence of statements with an optional return expression
    ///
    /// ### Arguments
    ///
    /// * `parser` - The parser instance
    ///
    /// ### Returns
    ///
    /// The parsed block expression or an error message
    pub fn parse_block_expression(parser: &mut Parser) -> Result<BlockExpr, ParseError> {
        let mut statements = Vec::new();
        let mut return_expr: Option<Expression> = None;

        while !parser.check(&Tokentype::RightBrace) && !parser.is_at_end() {
            let checkpoint = parser.current;

            if let Ok(expr) = parser.expression() {
                if parser.check(&Tokentype::RightBrace) {
                    return_expr = Some(expr);
                    break;
                } else if parser.match_token(&Tokentype::Semicolon) {
                    statements.push(StmtFactory::expression(expr));
                } else {
                    parser.current = checkpoint;
                    statements.push(parser.statement()?);
                }
            } else {
                parser.current = checkpoint;
                statements.push(parser.statement()?);
            }
        }

        if !parser.match_token(&Tokentype::RightBrace) {
            return Err(parser.error(ErrorCode::ExpectedClosingBrace, "Expected '}' after block"));
        }

        // Use factory for automatic location calculation from statements and return expression
        let block_expr = ExprFactory::block(statements, return_expr);
        
        // Extract the BlockExpr from the Expression::Block wrapper
        match block_expr {
            Expression::Block(block) => Ok(block),
            _ => unreachable!("ExprFactory::block should always return Expression::Block"),
        }
    }
}
