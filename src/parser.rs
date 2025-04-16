use crate::ast::{BinaryExpr, Expression, LetStatement, LiteralExpr, Statement, Type, Value, UnaryExpr};
use crate::token::{Token, Tokentype};
use std::collections::HashMap;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    type_map: HashMap<String, Type>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        let mut type_map = HashMap::new();
        // Register all built-in types
        type_map.insert("String".to_string(), Type::String);
        type_map.insert("i32".to_string(), Type::I32);
        type_map.insert("i64".to_string(), Type::I64);
        type_map.insert("u32".to_string(), Type::U32);
        type_map.insert("u64".to_string(), Type::U64);

        Parser {
            tokens,
            current: 0,
            type_map,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, String> {
        let stmt = if self.match_token(Tokentype::Let) {
            self.let_statement()?
        } else {
            self.expression_statement()?
        };

        if !self.match_token(Tokentype::Semicolon) {
            return Err("Expected semicolon at end of statement".to_string());
        }

        Ok(stmt)
    }

    fn let_statement(&mut self) -> Result<Statement, String> {
        if !self.check(Tokentype::Identifier) {
            return Err("Expected identifier after 'let'".to_string());
        }

        let token = self.advance();
        let name = token.value.to_string();
        let mut var_type = Type::Unknown; // Default to unknown, will be inferred

        // Check for type annotation (let x: int = ...)
        if self.match_token(Tokentype::Colon) {
            if !self.check(Tokentype::Identifier) {
                return Err("Expected type name after colon".to_string());
            }

            // Get the type name
            let type_token = self.advance();
            let type_name = type_token.value.to_string();

            // Look up the type in our registry
            if let Some(type_value) = self.type_map.get(&type_name) {
                var_type = type_value.clone();
            } else {
                return Err(format!("Unknown type: {}", type_name));
            }
        }

        if !self.match_token(Tokentype::Equal) {
            return Err("Expected '=' after variable name".to_string());
        }

        let expr = self.expression()?;

        Ok(Statement::Let(LetStatement {
            name,
            value: expr,
            expr_type: var_type,
        }))
    }

    fn expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.expression()?;
        Ok(Statement::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expression, String> {
        self.term()
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_token(Tokentype::Minus) {
            let operator = self.previous().token_type.clone();
            let right = self.primary()?;
            return Ok(Expression::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
                expr_type: Type::Unknown, // Will be inferred by type checker
            }));
        }

        self.primary()
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut expr = self.factor()?;

        while self.match_any(&[Tokentype::Plus, Tokentype::Minus]) {
            let operator = self.previous().token_type.clone();
            let right = self.factor()?;
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: Type::Unknown, // Default to unknown, will be inferred
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.unary()?;

        while self.match_any(&[Tokentype::Multiply, Tokentype::Divide]) {
            let operator = self.previous().token_type.clone();
            let right = self.primary()?;
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: Type::Unknown, // Default to unknown, will be inferred
            });
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_token(Tokentype::IntegerLiteral) {
            let value_str = self.previous().value.clone();
            let base_value = value_str
                .parse::<i64>()
                .map_err(|_| format!("Invalid integer: {}", value_str))?;

            // Check if the next token is an identifier that could be a type suffix
            if self.check(Tokentype::Identifier) {
                let type_name = self.peek().value.clone();

                // If it's a valid type suffix, consume it and create the appropriate literal
                match type_name.as_str() {
                    "i32" => {
                        self.advance(); // Consume the type token
                        // Check if value fits in i32
                        if base_value > i32::MAX as i64 || base_value < i32::MIN as i64 {
                            return Err(format!("Value {} is out of range for i32", base_value));
                        }
                        return Ok(Expression::Literal(LiteralExpr {
                            value: Value::I32(base_value as i32),
                            expr_type: Type::I32,
                        }));
                    }
                    "i64" => {
                        self.advance(); // Consume the type token
                        return Ok(Expression::Literal(LiteralExpr {
                            value: Value::I64(base_value),
                            expr_type: Type::I64,
                        }));
                    }
                    "u32" => {
                        self.advance(); // Consume the type token
                        // Check if value fits in u32
                        if base_value < 0 || base_value > u32::MAX as i64 {
                            return Err(format!("Value {} is out of range for u32", base_value));
                        }
                        return Ok(Expression::Literal(LiteralExpr {
                            value: Value::U32(base_value as u32),
                            expr_type: Type::U32,
                        }));
                    }
                    "u64" => {
                        self.advance(); // Consume the type token
                        // Check if value fits in u64
                        if base_value < 0 {
                            return Err(format!("Value {} is out of range for u64", base_value));
                        }
                        return Ok(Expression::Literal(LiteralExpr {
                            value: Value::U64(base_value as u64),
                            expr_type: Type::U64,
                        }));
                    }
                    _ => {} // Not a type suffix, treat as regular integer
                }
            }
            // If no type suffix, use default integer type
            return Ok(Expression::Literal(LiteralExpr {
                value: Value::I32(base_value as i32),
                expr_type: Type::I32,
            }));
        }

        if self.match_token(Tokentype::StringLiteral) {
            let value = self.previous().value.clone();
            return Ok(Expression::Literal(LiteralExpr {
                value: Value::String(value),
                expr_type: Type::String,
            }));
        }

        if self.match_token(Tokentype::Identifier) {
            return Ok(Expression::Variable(self.previous().value.clone()));
        }

        Err(format!("Expected expression, found {:?}", self.peek()))
    }

    fn match_token(&mut self, token_type: Tokentype) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_any(&mut self, types: &[Tokentype]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: Tokentype) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
