// Expression parsing module
// Contains logic for parsing expressions with proper operator precedence

use super::core::Parser;
use super::error::ParseError;
use crate::token::Tokentype;
use slang_error::ErrorCode;
use slang_ir::ast::{
    BinaryExpr, BinaryOperator, Expression, LiteralExpr, LiteralValue, UnaryExpr, UnaryOperator,
    VariableExpr,
};
use slang_types::PrimitiveType;

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
            let left_location = expr.location();
            let right = Self::logical_and(parser)?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);

            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
                expr_type: PrimitiveType::Bool.into(),
                location: span_location,
            });
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
            let left_location = expr.location();
            let right = Self::equality(parser)?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);

            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
                expr_type: PrimitiveType::Bool.into(),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses an equality expression (== and !=)
    ///
    /// ### Returns
    /// The parsed equality expression or an error message
    fn equality(parser: &mut Parser) -> Result<Expression, ParseError> {
        let mut expr = Self::comparison(parser)?;

        while parser.match_any(&[Tokentype::EqualEqual, Tokentype::NotEqual]) {
            let left_location = expr.location();
            let token = parser.previous();
            let operator = match token.token_type {
                Tokentype::EqualEqual => BinaryOperator::Equal,
                Tokentype::NotEqual => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };
            let right = Self::comparison(parser)?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);

            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: PrimitiveType::Bool.into(),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses a comparison expression (>, <, >=, <=)
    ///
    /// ### Returns
    /// The parsed comparison expression or an error message
    fn comparison(parser: &mut Parser) -> Result<Expression, ParseError> {
        let mut expr = Self::term(parser)?;

        while parser.match_any(&[
            Tokentype::Greater,
            Tokentype::GreaterEqual,
            Tokentype::Less,
            Tokentype::LessEqual,
        ]) {
            let left_location = expr.location();
            let token = parser.previous();
            let operator = match token.token_type {
                Tokentype::Greater => BinaryOperator::GreaterThan,
                Tokentype::GreaterEqual => BinaryOperator::GreaterThanOrEqual,
                Tokentype::Less => BinaryOperator::LessThan,
                Tokentype::LessEqual => BinaryOperator::LessThanOrEqual,
                _ => unreachable!(),
            };
            let right = Self::term(parser)?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);

            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: PrimitiveType::Bool.into(),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses a term expression (addition/subtraction)
    ///
    /// ### Returns
    /// The parsed term expression or an error message
    fn term(parser: &mut Parser) -> Result<Expression, ParseError> {
        let mut expr = Self::factor(parser)?;

        while parser.match_any(&[Tokentype::Plus, Tokentype::Minus]) {
            let left_location = expr.location();
            let token = parser.previous();
            let operator = match token.token_type {
                Tokentype::Plus => BinaryOperator::Add,
                Tokentype::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            let right = Self::factor(parser)?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);

            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: PrimitiveType::Unknown.into(),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses a factor expression (multiplication/division)
    ///
    /// ### Returns
    /// The parsed factor expression or an error message
    fn factor(parser: &mut Parser) -> Result<Expression, ParseError> {
        let mut expr = Self::unary(parser)?;

        while parser.match_any(&[Tokentype::Multiply, Tokentype::Divide]) {
            let left_location = expr.location();
            let token = parser.previous();
            let operator = match token.token_type {
                Tokentype::Multiply => BinaryOperator::Multiply,
                Tokentype::Divide => BinaryOperator::Divide,
                _ => unreachable!(),
            };
            let right = Self::unary(parser)?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);

            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: PrimitiveType::Unknown.into(),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses a unary expression (-, !)
    ///
    /// ### Returns
    /// The parsed unary expression or an error message
    fn unary(parser: &mut Parser) -> Result<Expression, ParseError> {
        if parser.match_token(&Tokentype::Minus) {
            let token = parser.previous();
            let operator_location = parser.source_location_from_token(token);
            let right = Self::primary(parser)?;
            let right_location = right.location();
            let span_location = operator_location.span_to(&right_location);

            return Ok(Expression::Unary(UnaryExpr {
                operator: UnaryOperator::Negate,
                right: Box::new(right),
                expr_type: PrimitiveType::Unknown.into(),
                location: span_location,
            }));
        }

        if parser.match_token(&Tokentype::Not) {
            let token = parser.previous();
            let operator_location = parser.source_location_from_token(token);
            let right = Self::primary(parser)?;
            let right_location = right.location();
            let span_location = operator_location.span_to(&right_location);

            return Ok(Expression::Unary(UnaryExpr {
                operator: UnaryOperator::Not,
                right: Box::new(right),
                expr_type: PrimitiveType::Bool.into(),
                location: span_location,
            }));
        }

        Self::primary(parser)
    }

    /// Parses a primary expression (literals, variables, grouped expressions)
    /// This has the highest precedence
    ///
    /// ### Returns
    /// The parsed primary expression or an error message
    fn primary(parser: &mut Parser) -> Result<Expression, ParseError> {
        if parser.match_token(&Tokentype::IntegerLiteral) {
            return parser.parse_integer();
        }

        if parser.match_token(&Tokentype::FloatLiteral) {
            return parser.parse_float();
        }

        if parser.match_token(&Tokentype::StringLiteral) {
            let token = parser.previous();
            let value = token.lexeme.clone();
            return Ok(Expression::Literal(LiteralExpr {
                value: LiteralValue::String(value),
                expr_type: PrimitiveType::String.into(),
                location: parser.source_location_from_token(token),
            }));
        }

        if parser.match_token(&Tokentype::BooleanLiteral) {
            let token = parser.previous();
            let lexeme = token.lexeme.clone();
            let bool_value = lexeme == "true";
            return Ok(Expression::Literal(LiteralExpr {
                value: LiteralValue::Boolean(bool_value),
                expr_type: PrimitiveType::Bool.into(),
                location: parser.source_location_from_token(token),
            }));
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
                parser.advance(); // consume the right paren
                let end_pos = parser.previous().pos + parser.previous().lexeme.len();
                let (line, column) = parser.line_info.get_line_col(start_pos);
                let location =
                    slang_ir::location::Location::new(start_pos, line, column, end_pos - start_pos);
                return Ok(Expression::Literal(LiteralExpr {
                    value: LiteralValue::Unit,
                    expr_type: PrimitiveType::Unit.into(),
                    location,
                }));
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

        if parser.match_token(&Tokentype::Identifier) {
            let name = parser.previous().lexeme.clone();

            if parser.match_token(&Tokentype::LeftParen) {
                return parser.finish_call(name);
            }

            let token = parser.previous();
            let location = parser.source_location_from_token(token);
            return Ok(Expression::Variable(VariableExpr { name, location }));
        }

        Err(parser.error(
            ErrorCode::ExpectedExpression,
            &format!("Expected expression, found {}", parser.peek()),
        ))
    }
}
