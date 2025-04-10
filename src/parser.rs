use crate::token::{Tokentype, Token};
use crate::ast::{Expression, LiteralExpr, Statement, Type, Value, BinaryExpr, LetStatement};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
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
            if self.match_token(Tokentype::TypeInt) {
                var_type = Type::Integer;
            } else if self.match_token(Tokentype::TypeString) {
                var_type = Type::String;
            } else {
                return Err("Expected type after colon".to_string());
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
        let mut expr = self.primary()?;

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
            let value = value_str
                .parse::<i64>()
                .map_err(|_| format!("Invalid integer: {}", value_str))?;
            return Ok(Expression::Literal(LiteralExpr {
                value: Value::Integer(value),
                expr_type: Type::Integer,
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

