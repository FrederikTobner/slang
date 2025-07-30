// Expression parsing module
// Contains logic for parsing expressions with operator precedence

use super::core::Parser;
use super::error::ParseError;
use crate::token::Tokentype;
use slang_error::ErrorCode;
use slang_ir::ast::{BinaryOperator, Expression, UnaryOperator, BlockExpr};
use slang_ir::ExprFactory; // Import factory system

/// Expression parser that handles operator precedence parsing
///
/// This parser uses the precedence climbing method to parse expressions
/// according to the language's operator precedence rules:
/// 1. Logical OR (||) - lowest precedence
/// 2. Logical AND (&&)
/// 3. Equality (==, !=)
/// 4. Comparison (>, <, >=, <=)
/// 5. Term (+, -)
/// 6. Factor (*, /)
/// 7. Unary (-, !)
/// 8. Primary (literals, variables, function calls, etc.) - highest precedence
pub struct ExpressionParser;

impl ExpressionParser {
    /// Entry point for parsing expressions
    /// Delegates to logical_or which has the lowest precedence
    ///
    /// ### Arguments
    /// * `parser` - Reference to the core parser
    ///
    /// ### Returns
    /// The parsed expression or an error message
    pub fn parse_expression(parser: &mut Parser) -> Result<Expression, ParseError> {
        Self::logical_or(parser)
    }

    /// Parses a logical OR expression (lowest precedence)
    ///
    /// ### Returns
    /// The parsed logical OR expression or an error message
    fn logical_or(parser: &mut Parser) -> Result<Expression, ParseError> {
        let mut expr = Self::logical_and(parser)?;

        while parser.match_token(&Tokentype::Or) {
            let right = Self::logical_and(parser)?;
            expr = ExprFactory::binary(expr, BinaryOperator::Or, right);
        }

        Ok(expr)
    }

    /// Parses a logical AND expression
    ///
    /// ### Returns
    /// The parsed logical AND expression or an error message
    fn logical_and(parser: &mut Parser) -> Result<Expression, ParseError> {
        let mut expr = Self::equality(parser)?;

        while parser.match_token(&Tokentype::And) {
            let right = Self::equality(parser)?;
            expr = ExprFactory::binary(expr, BinaryOperator::And, right);
        }

        Ok(expr)
    }

    /// Parses an equality expression (== and !=)
    ///
    /// ### Returns
    /// The parsed equality expression or an error message
    fn equality(parser: &mut Parser) -> Result<Expression, ParseError> {
        let mut expr = Self::comparison(parser)?;

        while let Some((operator, _position)) = parser.match_equality_operator() {
            let right = Self::comparison(parser)?;
            expr = ExprFactory::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// Parses a comparison expression (>, <, >=, <=)
    ///
    /// ### Returns
    /// The parsed comparison expression or an error message
    fn comparison(parser: &mut Parser) -> Result<Expression, ParseError> {
        let mut expr = Self::term(parser)?;

        while let Some((operator, _position)) = parser.match_comparison_operator() {
            let right = Self::term(parser)?;
            expr = ExprFactory::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// Parses a term expression (addition/subtraction)
    ///
    /// ### Returns
    /// The parsed term expression or an error message
    fn term(parser: &mut Parser) -> Result<Expression, ParseError> {
        let mut expr = Self::factor(parser)?;

        while let Some((operator, _position)) = parser.match_term_operator() {
            let right = Self::factor(parser)?;
            expr = ExprFactory::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// Parses a factor expression (multiplication/division)
    ///
    /// ### Returns
    /// The parsed factor expression or an error message
    fn factor(parser: &mut Parser) -> Result<Expression, ParseError> {
        let mut expr = Self::unary(parser)?;

        while let Some((operator, _position)) = parser.match_factor_operator() {
            let right = Self::unary(parser)?;
            expr = ExprFactory::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// Parses a unary expression (-, !)
    ///
    /// ### Returns
    /// The parsed unary expression or an error message
    fn unary(parser: &mut Parser) -> Result<Expression, ParseError> {
        if parser.match_token(&Tokentype::Minus) {
            let right = Self::primary(parser)?;
            return Ok(ExprFactory::unary(UnaryOperator::Negate, right));
        }

        if parser.match_token(&Tokentype::Not) {
            let right = Self::primary(parser)?;
            return Ok(ExprFactory::unary(UnaryOperator::Not, right));
        }

        Self::primary(parser)
    }

    /// Parses a primary expression (literals, variables, grouped expressions)
    /// This has the highest precedence
    ///
    /// ### Returns
    /// The parsed primary expression or an error message
    fn primary(parser: &mut Parser) -> Result<Expression, ParseError> {
        if parser.match_integer_literal_token() {
            return parser.parse_integer();
        }

        if parser.match_token(&Tokentype::FloatLiteral) {
            return parser.parse_float();
        }

        if let Some((value, position)) = parser.match_string_literal_token() {
            let value_string = value.to_string();
            let start_pos = position.pos;
            let end_pos = position.end_pos();
            let location = parser.location_from_range(start_pos, end_pos);
            return Ok(Expression::Literal(ExprFactory::literal_expr_with_location(
                value_string,
                location
            )));
        }

        if let Some((lexeme, position)) = parser.match_boolean_literal_token() {
            let lexeme_string = lexeme.to_string();
            let bool_value = lexeme_string == "true";
            let start_pos = position.pos;
            let end_pos = position.end_pos();
            let location = parser.location_from_range(start_pos, end_pos);
            return Ok(Expression::Literal(ExprFactory::literal_expr_with_location(
                bool_value,
                location
            )));
        }

        if parser.match_token(&Tokentype::If) {
            return parser.conditional_expression();
        }

        if parser.match_token(&Tokentype::Fn) {
            return parser.parse_function_type_expression();
        }

        if parser.match_token(&Tokentype::LeftParen) {
            // Check for unit literal ()
            if parser.check(&Tokentype::RightParen) {
                let start_pos = parser.previous().pos;
                let right_paren = parser.advance(); // consume the right paren
                let end_pos = right_paren.pos + right_paren.lexeme.len();
                let location = parser.location_from_range(start_pos, end_pos);
                return Ok(Expression::Literal(ExprFactory::literal_expr_with_location((), location)));
            }

            let expr = parser.expression()?;
            if !parser.match_token(&Tokentype::RightParen) {
                return Err(parser.error(
                    ErrorCode::ExpectedClosingParen,
                    "Expected ')' after expression",
                ));
            }
            return Ok(expr);
        }

        if parser.match_token(&Tokentype::LeftBrace) {
            let blockexpr = parser.parse_block_expression()?;
            return Ok(Expression::Block(blockexpr));
        }

        if let Some((name, position)) = parser.match_identifier_token() {
            let name_string = name.to_string();
            let start_pos = position.pos;
            let end_pos = position.end_pos();
            let name_location = parser.location_from_range(start_pos, end_pos);

            if parser.match_token(&Tokentype::LeftParen) {
                return parser.finish_call(name_string, name_location);
            }

            return Ok(Expression::Variable(ExprFactory::variable_expr_with_location(name_string, name_location)));
        }

        Err(parser.error(
            ErrorCode::ExpectedExpression,
            &format!("Expected expression, found {}", parser.peek()),
        ))
    }

    /// Finishes parsing a function call after the name and '('
    ///
    /// ### Arguments
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
        Ok(Expression::Conditional(ExprFactory::conditional_expr(
            condition,
            Expression::Block(then_branch),
            Expression::Block(else_branch),
        )))
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
                    statements.push(slang_ir::ast::Statement::Expression(expr));
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
        return Ok(ExprFactory::block_expr(statements, return_expr));
    }
}
