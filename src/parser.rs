use crate::ast::{
    BinaryExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt, LetStatement, LiteralExpr, Parameter, 
    Statement, TypeDefinitionStmt, UnaryExpr, Value,
};
use crate::token::{Token, Tokentype};
use crate::types::{TypeId, TYPE_REGISTRY};
use crate::types::{i32_type, i64_type, u32_type, u64_type, f32_type, f64_type, string_type, 
                  unspecified_int_type, unspecified_float_type, unknown_type, bool_type};

/// Error that occurs during parsing
#[derive(Debug)]
pub struct ParseError {
    /// Error message describing the problem
    message: String,
}

impl ParseError {
    /// Creates a new parse error with the given message
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

/// Parser that converts tokens into an abstract syntax tree
pub struct Parser<'a> {
    /// The tokens being parsed
    tokens: &'a [Token],
    /// Current position in the token list
    current: usize,
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given tokens
    /// 
    /// # Arguments
    /// 
    /// * `tokens` - The tokens to parse
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    /// Parses the tokens into a list of statements
    /// 
    /// # Returns
    /// 
    /// The parsed statements or an error message
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

    /// Parses a single statement
    /// 
    /// # Returns
    /// 
    /// The parsed statement or an error message
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
    
    /// Parses a block statement (a group of statements in braces)
    /// 
    /// # Returns
    /// 
    /// The parsed block statement or an error message
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
    
    /// Parses a return statement
    /// 
    /// # Returns
    /// 
    /// The parsed return statement or an error message
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
    
    /// Parses a function declaration
    /// 
    /// # Returns
    /// 
    /// The parsed function declaration or an error message
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
            unknown_type()
        };
        
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
    
    /// Parses a function parameter
    /// 
    /// # Returns
    /// 
    /// The parsed parameter or an error message
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

    /// Parses a type definition (struct declaration)
    /// 
    /// # Returns
    /// 
    /// The parsed type definition or an error message
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
        
        while !self.check(Tokentype::RightBrace) && !self.is_at_end() {
            if !self.check(Tokentype::Identifier) {
                return Err("Expected field name".to_string());
            }
            let field_name = self.advance().lexeme.clone();
            
            if !self.match_token(Tokentype::Colon) {
                return Err("Expected ':' after field name".to_string());
            }
            
            let field_type = match self.parse_type() {
                Ok(t) => t,
                Err(e) => return Err(e.to_string()),
            };
            
            fields.push((field_name, field_type));
            
            if !self.match_token(Tokentype::Comma) && !self.check(Tokentype::RightBrace) {
                return Err("Expected ',' after field or '}'".to_string());
            }
        }
        
        if !self.match_token(Tokentype::RightBrace) {
            return Err("Expected '}' after struct fields".to_string());
        }
        
        if !self.match_token(Tokentype::Semicolon) {
            return Err("Expected ';' after struct definition".to_string());
        }
        
        Ok(Statement::TypeDefinition(TypeDefinitionStmt { name, fields }))
    }

    /// Parses a variable declaration
    /// 
    /// # Returns
    /// 
    /// The parsed variable declaration or an error message
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
            
            // Explicitly reject placeholder types as type specifiers
            if type_name == "int" {
                return Err("'int' is not a valid type specifier. Use 'i32', 'i64', 'u32', or 'u64' instead".to_string());
            } else if type_name == "float" {
                return Err("'float' is not a valid type specifier. Use 'f32' or 'f64' instead".to_string());
            }
            
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

    /// Parses an expression statement
    /// 
    /// # Returns
    /// 
    /// The parsed expression statement or an error message
    fn expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.expression()?;
        
        if !self.match_token(Tokentype::Semicolon) {
            return Err("Expected ';' after expression".to_string());
        }
        
        Ok(Statement::Expression(expr))
    }

    /// Parses an expression
    /// 
    /// # Returns
    /// 
    /// The parsed expression or an error message
    fn expression(&mut self) -> Result<Expression, String> {
        self.term()
    }

    /// Parses a term (addition/subtraction)
    /// 
    /// # Returns
    /// 
    /// The parsed term or an error message
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

    /// Parses a factor (multiplication/division)
    /// 
    /// # Returns
    /// 
    /// The parsed factor or an error message
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

    /// Parses a unary expression
    /// 
    /// # Returns
    /// 
    /// The parsed unary expression or an error message
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
        
        if self.match_token(Tokentype::Not) {
            let operator = self.previous().token_type.clone();
            let right = self.primary()?;
            return Ok(Expression::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
                expr_type: bool_type(),
            }));
        }

        self.primary()
    }

    /// Parses a primary expression (literal, variable, or grouped expression)
    /// 
    /// # Returns
    /// 
    /// The parsed primary expression or an error message
    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_token(Tokentype::IntegerLiteral) {
            return self.parse_integer();
        }

        if self.match_token(Tokentype::FloatLiteral) {
            return self.parse_float();
        }

        if self.match_token(Tokentype::StringLiteral) {
            let value = self.previous().lexeme.clone();
            return Ok(Expression::Literal(LiteralExpr {
                value: Value::String(value),
                expr_type: string_type(),
            }));
        }
        
        if self.match_token(Tokentype::BooleanLiteral) {
            let lexeme = self.previous().lexeme.clone();
            let bool_value = lexeme == "true";
            return Ok(Expression::Literal(LiteralExpr {
                value: Value::Boolean(bool_value),
                expr_type: bool_type(),
            }));
        }

        if self.match_token(Tokentype::LeftParen) {
            let expr = self.expression()?;
            if !self.match_token(Tokentype::RightParen) {
                return Err("Expected ')' after expression".to_string());
            }
            return Ok(expr);
        }

        if self.match_token(Tokentype::Identifier) {
            let name = self.previous().lexeme.clone();
            
            if self.match_token(Tokentype::LeftParen) {
                return self.finish_call(name);
            }
            
            return Ok(Expression::Variable(name));
        }

        Err(format!("Expected expression, found {:?}", self.peek()))
    }
    
fn parse_float(&mut self) -> Result<Expression, String> {
        let value_str = self.previous().lexeme.clone();
        let value = value_str
            .parse::<f64>()
            .map_err(|_| format!("Invalid float: {}", value_str))?;
        
        if self.check(Tokentype::Identifier) {
            let type_name = self.peek().lexeme.clone();
            
            match type_name.as_str() {
                "f32" => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: Value::F32(value as f32),
                        expr_type: f32_type(),
                    }));
                }
                "f64" => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: Value::F64(value),
                        expr_type: f64_type(),
                    }));
                }
                _ => {}
            }
        }
        
        // Unspecified float literal
        Ok(Expression::Literal(LiteralExpr {
            value: Value::UnspecifiedFloat(value),
            expr_type: unspecified_float_type(),
        }))
    }

    /// Finishes parsing a function call after the name and '('
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the function being called
    /// 
    /// # Returns
    /// 
    /// The parsed function call expression or an error message
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

    /// Parses an integer literal with optional type suffix
    /// 
    /// # Returns
    /// 
    /// The parsed integer literal expression or an error message
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
                "f32" => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: Value::F32(base_value as f32),
                        expr_type: f32_type(),
                    }));
                }
                "f64" => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: Value::F64(base_value as f64),
                        expr_type: f64_type(),
                    }));
                }
                _ => {}
            }
        }
        
        // Unspecified integer literal
        Ok(Expression::Literal(LiteralExpr {
            value: Value::UnspecifiedInteger(base_value),
            expr_type: unspecified_int_type(),
        }))
    }

    /// Parses a type name
    /// 
    /// # Returns
    /// 
    /// The type ID for the parsed type or an error
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

    /// Consumes the current token if it matches the expected type
    /// 
    /// # Arguments
    /// 
    /// * `token_type` - The token type to match
    /// 
    /// # Returns
    /// 
    /// true if the token was consumed, false otherwise
    fn match_token(&mut self, token_type: Tokentype) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consumes the current token if it matches any of the expected types
    /// 
    /// # Arguments
    /// 
    /// * `types` - The token types to match
    /// 
    /// # Returns
    /// 
    /// true if a token was consumed, false otherwise
    fn match_any(&mut self, types: &[Tokentype]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Checks if the current token is of the expected type
    /// 
    /// # Arguments
    /// 
    /// * `token_type` - The token type to check for
    /// 
    /// # Returns
    /// 
    /// true if the current token matches, false otherwise
    fn check(&self, token_type: Tokentype) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    /// Advances to the next token and returns the previous token
    /// 
    /// # Returns
    /// 
    /// The token that was current before advancing
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Checks if we've reached the end of the token stream
    /// 
    /// # Returns
    /// 
    /// true if we're at the end, false otherwise
    fn is_at_end(&self) -> bool {
        self.peek().token_type == Tokentype::Eof
    }

    /// Returns the current token without consuming it
    /// 
    /// # Returns
    /// 
    /// The current token
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Returns the most recently consumed token
    /// 
    /// # Returns
    /// 
    /// The previous token
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
