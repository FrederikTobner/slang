// Expression parsing module
// Contains logic for parsing expressions with operator precedence

use super::core::Parser;
use crate::token::Tokentype;
use slang_error::{ParseError, ParseErrorFactory};
use slang_ir::ExprFactory;
use slang_ir::ast::{BinaryOperator, BlockExpr, Expression, UnaryOperator};

/// Extension trait for expression parsing functionality
///
/// This trait extends the Parser with expression parsing methods,
/// using the precedence climbing method to parse expressions
/// according to the language's operator precedence rules:
/// 1. Logical OR (||) - lowest precedence
/// 2. Logical AND (&&)
/// 3. Equality (==, !=)
/// 4. Comparison (>, <, >=, <=)
/// 5. Term (+, -)
/// 6. Factor (*, /)
/// 7. Unary (-, !)
/// 8. Primary (literals, variables, function calls, etc.) - highest precedence
pub trait ExpressionParsing {
    /// Entry point for parsing expressions
    fn expression(&mut self) -> Result<Expression, ParseError>;

    /// Parse logical OR expressions (lowest precedence)
    fn logical_or(&mut self) -> Result<Expression, ParseError>;

    /// Parse logical AND expressions
    fn logical_and(&mut self) -> Result<Expression, ParseError>;

    /// Parse equality expressions (==, !=)
    fn equality(&mut self) -> Result<Expression, ParseError>;

    /// Parse comparison expressions (>, <, >=, <=)
    fn comparison(&mut self) -> Result<Expression, ParseError>;

    /// Parse term expressions (+, -)
    fn term(&mut self) -> Result<Expression, ParseError>;

    /// Parse factor expressions (*, /)
    fn factor(&mut self) -> Result<Expression, ParseError>;

    /// Parse unary expressions (-, !)
    fn unary(&mut self) -> Result<Expression, ParseError>;

    /// Parse primary expressions (literals, variables, grouped)
    fn primary(&mut self) -> Result<Expression, ParseError>;

    /// Finish parsing a function call after name and '('
    fn finish_call(
        &mut self,
        name: String,
        name_location: slang_error::location::Location,
    ) -> Result<Expression, ParseError>;

    /// Parse conditional expressions (if/else)
    fn conditional_expression(&mut self) -> Result<Expression, ParseError>;

    /// Parse block expressions
    fn parse_block_expression(&mut self) -> Result<BlockExpr, ParseError>;
}

impl<'a> ExpressionParsing for Parser<'a> {
    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.logical_or()
    }

    fn logical_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.logical_and()?;

        while self.match_token(&Tokentype::Or) {
            let right = self.logical_and()?;
            expr = ExprFactory::binary(expr, BinaryOperator::Or, right);
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token(&Tokentype::And) {
            let right = self.equality()?;
            expr = ExprFactory::binary(expr, BinaryOperator::And, right);
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;

        while let Some((operator, _position)) = self.match_equality_operator() {
            let right = self.comparison()?;
            expr = ExprFactory::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;

        while let Some((operator, _position)) = self.match_comparison_operator() {
            let right = self.term()?;
            expr = ExprFactory::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;

        while let Some((operator, _position)) = self.match_term_operator() {
            let right = self.factor()?;
            expr = ExprFactory::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;

        while let Some((operator, _position)) = self.match_factor_operator() {
            let right = self.unary()?;
            expr = ExprFactory::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        if self.match_token(&Tokentype::Minus) {
            let right = self.primary()?;
            return Ok(ExprFactory::unary(UnaryOperator::Negate, right));
        }

        if self.match_token(&Tokentype::Not) {
            let right = self.primary()?;
            return Ok(ExprFactory::unary(UnaryOperator::Not, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        if self.match_integer_literal_token() {
            return self.parse_integer();
        }

        if self.match_token(&Tokentype::FloatLiteral) {
            return self.parse_float();
        }

        if let Some((value, position)) = self.match_string_literal_token() {
            let value_string = value.to_string();
            let start_pos = position.pos;
            let end_pos = position.end_pos();
            let location = self.location_from_range(start_pos, end_pos);
            return Ok(Expression::Literal(
                ExprFactory::literal_expr_with_location(value_string, location),
            ));
        }

        if let Some((lexeme, position)) = self.match_boolean_literal_token() {
            let lexeme_string = lexeme.to_string();
            let bool_value = lexeme_string == "true";
            let start_pos = position.pos;
            let end_pos = position.end_pos();
            let location = self.location_from_range(start_pos, end_pos);
            return Ok(Expression::Literal(
                ExprFactory::literal_expr_with_location(bool_value, location),
            ));
        }

        if self.match_token(&Tokentype::If) {
            return self.conditional_expression();
        }

        if self.match_token(&Tokentype::Fn) {
            return self.parse_function_type_expression();
        }

        if self.match_token(&Tokentype::LeftParen) {
            // Check for unit literal ()
            if self.check(&Tokentype::RightParen) {
                let start_pos = self.previous().pos;
                let right_paren = self.advance();
                let end_pos = right_paren.pos + right_paren.lexeme.len();
                let location = self.location_from_range(start_pos, end_pos);
                return Ok(Expression::Literal(
                    ExprFactory::literal_expr_with_location((), location),
                ));
            }

            let expr = self.expression()?;
            if !self.match_token(&Tokentype::RightParen) {
                return Err(ParseErrorFactory::expected_closing_paren(
                    self.current_location(),
                    Some("after expression"),
                ));
            }
            return Ok(expr);
        }

        if self.match_token(&Tokentype::LeftBrace) {
            let blockexpr = self.parse_block_expression()?;
            return Ok(Expression::Block(blockexpr));
        }

        if let Some((name, position)) = self.match_identifier_token() {
            let name_string = name.to_string();
            let start_pos = position.pos;
            let end_pos = position.end_pos();
            let name_location = self.location_from_range(start_pos, end_pos);

            if self.match_token(&Tokentype::LeftParen) {
                return self.finish_call(name_string, name_location);
            }

            return Ok(Expression::Variable(
                ExprFactory::variable_expr_with_location(name_string, name_location),
            ));
        }

        let token = self.peek();
        Err(ParseErrorFactory::invalid_syntax(
            self.current_location(),
            &format!("Expected expression, found {token}"),
            None,
        ))
    }

    fn finish_call(
        &mut self,
        name: String,
        name_location: slang_error::location::Location,
    ) -> Result<Expression, ParseError> {
        let mut arguments = Vec::new();

        if !self.check(&Tokentype::RightParen) {
            arguments.push(self.expression()?);

            while self.match_token(&Tokentype::Comma) {
                if arguments.len() >= 255 {
                    return Err(ParseErrorFactory::invalid_syntax(
                        self.current_location(),
                        "Cannot have more than 255 arguments",
                        None,
                    ));
                }
                arguments.push(self.expression()?);
            }
        }

        if !self.match_token(&Tokentype::RightParen) {
            return Err(ParseErrorFactory::expected_closing_paren(
                self.current_location(),
                Some("after function arguments"),
            ));
        }

        Ok(Expression::Call(
            slang_ir::ExprFactory::call_expr_with_location(name, arguments, name_location),
        ))
    }

    fn conditional_expression(&mut self) -> Result<Expression, ParseError> {
        let condition = self.expression()?;

        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(ParseErrorFactory::expected_opening_brace(
                self.current_location(),
                Some("after if condition"),
            ));
        }

        let then_branch = self.parse_block_expression()?;

        if !self.match_token(&Tokentype::Else) {
            return Err(ParseErrorFactory::expected_else_after_if(
                self.current_location(),
            ));
        }

        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(ParseErrorFactory::expected_opening_brace(
                self.current_location(),
                Some("after else"),
            ));
        }

        let else_branch = self.parse_block_expression()?;

        Ok(Expression::Conditional(ExprFactory::conditional_expr(
            condition,
            Expression::Block(then_branch),
            Expression::Block(else_branch),
        )))
    }

    fn parse_block_expression(&mut self) -> Result<BlockExpr, ParseError> {
        let mut statements = Vec::new();
        let mut return_expr: Option<Expression> = None;

        while !self.check(&Tokentype::RightBrace) && !self.is_at_end() {
            let checkpoint = self.current;

            if let Ok(expr) = self.expression() {
                if self.check(&Tokentype::RightBrace) {
                    return_expr = Some(expr);
                    break;
                } else if self.match_token(&Tokentype::Semicolon) {
                    statements.push(slang_ir::ast::Statement::Expression(expr));
                } else {
                    self.current = checkpoint;
                    statements.push(self.statement()?);
                }
            } else {
                self.current = checkpoint;
                statements.push(self.statement()?);
            }
        }

        if !self.match_token(&Tokentype::RightBrace) {
            return Err(ParseErrorFactory::expected_closing_brace(
                self.current_location(),
                Some("after block"),
            ));
        }

        Ok(ExprFactory::block_expr(statements, return_expr))
    }
}
