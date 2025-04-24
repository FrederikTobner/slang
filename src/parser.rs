use crate::ast::{
    BinaryExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt, LetStatement, LiteralExpr, Parameter, 
    Statement, TypeDefinitionStmt, UnaryExpr, Value,
};
use crate::token::{Token, Tokentype};
use crate::types::{TypeId, TYPE_REGISTRY};
use crate::types::{i32_type, i64_type, u32_type, u64_type, f64_type, string_type, unspecified_int_type, unknown_type};

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: &str) -> Self {
        ParseError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            current: 0,
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
        if self.match_token(Tokentype::Let) {
            return self.let_statement();
        } else if self.match_token(Tokentype::Struct) {
            return self.type_definition_statement();
        } else if self.match_token(Tokentype::Fn) {
            return self.function_declaration_statement();
        } else if self.match_token(Tokentype::Return) {
            return self.return_statement();
        } else if self.match_token(Tokentype::LeftBrace) {
            return self.block_statement();
        } else {
            return self.expression_statement();
        }
    }
    
    fn block_statement(&mut self) -> Result<Statement, String> {
        let mut statements = Vec::new();
        
        while !self.check(Tokentype::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }
        
        if !self.match_token(Tokentype::RightBrace) {
            return Err("Expected '}' after block".to_string());
        }
        
        Ok(Statement::Block(statements))
    }
    
    fn return_statement(&mut self) -> Result<Statement, String> {
        let value = if !self.check(Tokentype::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        
        if !self.match_token(Tokentype::Semicolon) {
            return Err("Expected ';' after return value".to_string());
        }
        
        Ok(Statement::Return(value))
    }
    
    fn function_declaration_statement(&mut self) -> Result<Statement, String> {
        // Parse function name
        if !self.check(Tokentype::Identifier) {
            return Err("Expected function name".to_string());
        }
        let name = self.advance().lexeme.clone();
        
        // Parse parameter list
        if !self.match_token(Tokentype::LeftParen) {
            return Err("Expected '(' after function name".to_string());
        }
        
        let mut parameters = Vec::new();
        if !self.check(Tokentype::RightParen) {
            // Parse first parameter
            parameters.push(self.parameter()?);
            
            // Parse the rest of the parameters
            while self.match_token(Tokentype::Comma) {
                if parameters.len() >= 255 {
                    return Err("Cannot have more than 255 parameters".to_string());
                }
                parameters.push(self.parameter()?);
            }
        }
        
        if !self.match_token(Tokentype::RightParen) {
            return Err("Expected ')' after parameters".to_string());
        }
        
        // Parse return type
        let return_type = if self.match_token(Tokentype::Arrow) {
            if !self.check(Tokentype::Identifier) {
                return Err("Expected return type after '->'".to_string());
            }
            
            let type_name = self.advance().lexeme.clone();
            TYPE_REGISTRY.with(|registry| {
                let registry = registry.borrow();
                registry.get_type_by_name(&type_name)
                    .cloned()
                    .unwrap_or_else(|| unknown_type())
            })
        } else {
            // Default to unknown/void type if no return type specified
            unknown_type()
        };
        
        // Parse function body
        if !self.match_token(Tokentype::LeftBrace) {
            return Err("Expected '{' before function body".to_string());
        }
        
        let mut body = Vec::new();
        while !self.check(Tokentype::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }
        
        if !self.match_token(Tokentype::RightBrace) {
            return Err("Expected '}' after function body".to_string());
        }
        
        Ok(Statement::FunctionDeclaration(FunctionDeclarationStmt {
            name,
            parameters,
            return_type,
            body,
        }))
    }
    
    fn parameter(&mut self) -> Result<Parameter, String> {
        if !self.check(Tokentype::Identifier) {
            return Err("Expected parameter name".to_string());
        }
        
        let name = self.advance().lexeme.clone();
        
        if !self.match_token(Tokentype::Colon) {
            return Err("Expected ':' after parameter name".to_string());
        }
        
        if !self.check(Tokentype::Identifier) {
            return Err("Expected parameter type".to_string());
        }
        
        let type_name = self.advance().lexeme.clone();
        let param_type = TYPE_REGISTRY.with(|registry| {
            let registry = registry.borrow();
            registry.get_type_by_name(&type_name)
                .cloned()
                .unwrap_or_else(|| unknown_type())
        });
        
        if param_type == unknown_type() {
            return Err(format!("Unknown type: {}", type_name));
        }
        
        Ok(Parameter { name, param_type })
    }

    fn type_definition_statement(&mut self) -> Result<Statement, String> {
        // Expect struct name
        if !self.check(Tokentype::Identifier) {
            return Err("Expected struct name after 'struct' keyword".to_string());
        }
        
        let name = self.advance().lexeme.clone();
        
        // Expect opening brace
        if !self.match_token(Tokentype::LeftBrace) {
            return Err("Expected '{' after struct name".to_string());
        }
        
        let mut fields = Vec::new();
        
        // Parse fields until closing brace
        while !self.check(Tokentype::RightBrace) && !self.is_at_end() {
            // Field name
            if !self.check(Tokentype::Identifier) {
                return Err("Expected field name".to_string());
            }
            let field_name = self.advance().lexeme.clone();
            
            // Field type
            if !self.match_token(Tokentype::Colon) {
                return Err("Expected ':' after field name".to_string());
            }
            
            let field_type = match self.parse_type() {
                Ok(t) => t,
                Err(e) => return Err(e.to_string()),
            };
            
            fields.push((field_name, field_type));
            
            // Expect comma or closing brace
            if !self.match_token(Tokentype::Comma) && !self.check(Tokentype::RightBrace) {
                return Err("Expected ',' after field or '}'".to_string());
            }
        }
        
        // Expect closing brace
        if !self.match_token(Tokentype::RightBrace) {
            return Err("Expected '}' after struct fields".to_string());
        }
        
        // Expect semicolon
        if !self.match_token(Tokentype::Semicolon) {
            return Err("Expected ';' after struct definition".to_string());
        }
        
        Ok(Statement::TypeDefinition(TypeDefinitionStmt { name, fields }))
    }

    fn let_statement(&mut self) -> Result<Statement, String> {
        if !self.check(Tokentype::Identifier) {
            return Err("Expected identifier after 'let'".to_string());
        }

        let token = self.advance();
        let name = token.lexeme.clone();
        let mut var_type = unknown_type();

        if self.match_token(Tokentype::Colon) {
            if !self.check(Tokentype::Identifier) {
                return Err("Expected type name after colon".to_string());
            }

            let type_name = self.advance().lexeme.clone();
            
            // Look up the type in the registry
            var_type = TYPE_REGISTRY.with(|registry| {
                let registry = registry.borrow();
                registry.get_type_by_name(&type_name)
                    .cloned()
                    .unwrap_or_else(|| unknown_type())
            });
            
            if var_type == unknown_type() {
                return Err(format!("Unknown type: {}", type_name));
            }
        }

        if !self.match_token(Tokentype::Equal) {
            return Err("Expected '=' after variable name".to_string());
        }

        let expr = self.expression()?;

        // Expect semicolon
        if !self.match_token(Tokentype::Semicolon) {
            return Err("Expected ';' after let statement".to_string());
        }

        Ok(Statement::Let(LetStatement {
            name,
            value: expr,
            expr_type: var_type,
        }))
    }

    fn expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.expression()?;
        
        // Expect semicolon
        if !self.match_token(Tokentype::Semicolon) {
            return Err("Expected ';' after expression".to_string());
        }
        
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
                expr_type: unknown_type(),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.unary()?;

        while self.match_any(&[Tokentype::Multiply, Tokentype::Divide]) {
            let operator = self.previous().token_type;
            let right = self.unary()?;
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: unknown_type(),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_token(Tokentype::Minus) {
            let operator = self.previous().token_type.clone();
            let right = self.primary()?;
            return Ok(Expression::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
                expr_type: unknown_type(),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_token(Tokentype::IntegerLiteral) {
            return self.parse_integer();
        }

        if self.match_token(Tokentype::FloatLiteral) {
            let value_str = self.previous().lexeme.clone();
            let value = value_str
                .parse::<f64>()
                .map_err(|_| format!("Invalid float: {}", value_str))?;
            return Ok(Expression::Literal(LiteralExpr {
                value: Value::F64(value),
                expr_type: f64_type(),
            }));
        }

        if self.match_token(Tokentype::StringLiteral) {
            let value = self.previous().lexeme.clone();
            return Ok(Expression::Literal(LiteralExpr {
                value: Value::String(value),
                expr_type: string_type(),
            }));
        }

        if self.match_token(Tokentype::Identifier) {
            let name = self.previous().lexeme.clone();
            
            // Check if this is a function call
            if self.match_token(Tokentype::LeftParen) {
                return self.finish_call(name);
            }
            
            return Ok(Expression::Variable(name));
        }

        Err(format!("Expected expression, found {:?}", self.peek()))
    }
    
    fn finish_call(&mut self, name: String) -> Result<Expression, String> {
        let mut arguments = Vec::new();
        
        if !self.check(Tokentype::RightParen) {
            // Parse first argument
            arguments.push(self.expression()?);
            
            // Parse rest of the arguments
            while self.match_token(Tokentype::Comma) {
                if arguments.len() >= 255 {
                    return Err("Cannot have more than 255 arguments".to_string());
                }
                arguments.push(self.expression()?);
            }
        }
        
        if !self.match_token(Tokentype::RightParen) {
            return Err("Expected ')' after function arguments".to_string());
        }
        
        Ok(Expression::Call(FunctionCallExpr {
            name,
            arguments,
            expr_type: unknown_type(), // Type will be determined during type checking
        }))
    }

    fn parse_integer(&mut self) -> Result<Expression, String> {
        let value_str = self.previous().lexeme.clone();
        let base_value = value_str
            .parse::<i64>()
            .map_err(|_| format!("Invalid integer: {}", value_str))?;

        if self.check(Tokentype::Identifier) {
            let type_name = self.peek().lexeme.clone();

            match type_name.as_str() {
                "i32" => {
                    self.advance();
                    if base_value > i32::MAX as i64 || base_value < i32::MIN as i64 {
                        return Err(format!("Value {} is out of range for i32", base_value));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: Value::I32(base_value as i32),
                        expr_type: i32_type(),
                    }));
                }
                "i64" => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: Value::I64(base_value),
                        expr_type: i64_type(),
                    }));
                }
                "u32" => {
                    self.advance();
                    if base_value < 0 || base_value > u32::MAX as i64 {
                        return Err(format!("Value {} is out of range for u32", base_value));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: Value::U32(base_value as u32),
                        expr_type: u32_type(),
                    }));
                }
                "u64" => {
                    self.advance();
                    if base_value < 0 {
                        return Err(format!("Value {} is out of range for u64", base_value));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: Value::U64(base_value as u64),
                        expr_type: u64_type(),
                    }));
                }
                _ => {}
            }
        }
        
        // Unspecified integer
        Ok(Expression::Literal(LiteralExpr {
            value: Value::UnspecifiedInteger(base_value),
            expr_type: unspecified_int_type(),
        }))
    }

    fn parse_type(&mut self) -> Result<TypeId, ParseError> {
        if !self.check(Tokentype::Identifier) {
            return Err(ParseError::new("Expected type identifier"));
        }
        
        let type_name = self.advance().lexeme.clone();
        
        TYPE_REGISTRY.with(|registry| {
            let registry = registry.borrow();
            if let Some(type_id) = registry.get_type_by_name(&type_name) {
                Ok(type_id.clone())
            } else {
                Err(ParseError::new(&format!("Unknown type: {}", type_name)))
            }
        })
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
        self.peek().token_type == Tokentype::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
