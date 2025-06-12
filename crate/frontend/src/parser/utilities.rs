// Utilities module
// Contains helper functions and common parsing patterns

use slang_error::ErrorCode;
use crate::token::Tokentype;
use super::error::ParseError;
use slang_ir::ast::{
    BlockExpr, ConditionalExpr, Expression, FunctionCallExpr, Statement,
};
use slang_types::PrimitiveType;
use super::core::Parser;

/// Utilities parser module providing static methods for common parsing patterns
pub struct UtilitiesParser;

impl UtilitiesParser {
    /// Finishes parsing a function call after the name and '('
    ///
    /// #### Arguments
    ///
    /// * `parser` - The parser instance
    /// * `name` - The name of the function being called
    ///
    /// ### Returns
    ///
    /// The parsed function call expression or an error message
    pub fn finish_call(parser: &mut Parser, name: String) -> Result<Expression, ParseError> {
        let name_token = parser.previous();
        let start_location = parser.source_location_from_token(name_token);

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

        let closing_paren_token = parser.previous();
        let end_location = parser.source_location_from_token(closing_paren_token);
        let span_location = start_location.span_to(&end_location);

        Ok(Expression::Call(FunctionCallExpr {
            name,
            arguments,
            expr_type: PrimitiveType::Unknown.into(),
            location: span_location,
        }))
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
        let if_token_pos = parser.previous().pos;
        let (line, column) = parser.line_info.get_line_col(if_token_pos);

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

        let end_pos = parser.previous().pos + parser.previous().lexeme.len();
        let location = slang_ir::location::Location::new(
            if_token_pos,
            line,
            column,
            end_pos - if_token_pos,
        );

        Ok(Expression::Conditional(ConditionalExpr {
            condition: Box::new(condition),
            then_branch: Box::new(Expression::Block(then_branch)),
            else_branch: Box::new(Expression::Block(else_branch)),
            expr_type: PrimitiveType::Unknown.into(),
            location,
        }))
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
        let start_pos = parser.current;
        let (line, column) = parser.line_info.get_line_col(parser.tokens[start_pos].pos);

        let mut statements = Vec::new();
        let mut return_expr: Option<Box<Expression>> = None;

        while !parser.check(&Tokentype::RightBrace) && !parser.is_at_end() {
            let checkpoint = parser.current;

            if let Ok(expr) = parser.expression() {
                if parser.check(&Tokentype::RightBrace) {
                    return_expr = Some(Box::new(expr));
                    break;
                } else if parser.match_token(&Tokentype::Semicolon) {
                    statements.push(Statement::Expression(expr));
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

        let end_pos = parser.previous().pos + parser.previous().lexeme.len();
        let location = slang_ir::location::Location::new(
            parser.tokens[start_pos].pos,
            line,
            column,
            end_pos - parser.tokens[start_pos].pos,
        );

        Ok(BlockExpr {
            statements,
            return_expr,
            expr_type: PrimitiveType::Unknown.into(),
            location,
        })
    }
}
