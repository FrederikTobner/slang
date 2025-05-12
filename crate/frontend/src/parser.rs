use slang_ir::ast::{
    BinaryExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt, LetStatement, LiteralExpr,
    LiteralValue, Parameter, Statement, TypeDefinitionStmt, UnaryExpr, UnaryOperator, BinaryOperator,
};
use crate::error::LineInfo;
use crate::token::{Token, Tokentype};
use slang_types::types::{TYPE_REGISTRY, TypeId};
use slang_types::types::{
    bool_type, f32_type, f64_type, i32_type, i64_type, string_type, u32_type, u64_type,
    unknown_type, unspecified_float_type, unspecified_int_type,
};
use crate::error::{CompilerError, CompileResult};

/// Error that occurs during parsing
#[derive(Debug)]
pub struct ParseError {
    /// Error message describing the problem
    message: String,
    /// Position in the source code where the error occurred
    position: usize,
    /// Length of the underlined part
    underline_length: usize,
}

impl ParseError {
    /// Creates a new parse error with the given message and position
    pub fn new(message: &str, position: usize, underline_length: usize) -> Self {
        ParseError {
            message: message.to_string(),
            position,
            underline_length,
        }
    }

    /// Format this error using line information
    pub fn format_with_line_info(&self, line_info: &LineInfo) -> String {
        line_info.format_error(self.position, &self.message, self.underline_length)
    }

    pub fn to_compiler_error(
        &self,
        line_info: &LineInfo,
    ) -> CompilerError {
        let line_pos = line_info.get_line_col(self.position);
        CompilerError::new(
            self.format_with_line_info(line_info),
            line_pos.0, 
            line_pos.1,
        )
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
    /// Line information for error reporting
    line_info: &'a LineInfo<'a>,
    /// Errors collected during parsing
    errors: Vec<CompilerError>,
}

pub fn parse(tokens: &[Token], line_info: &LineInfo) -> CompileResult<Vec<Statement>> {
    let mut parser = Parser::new(tokens, line_info);
    parser.parse()
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given tokens and line information
    ///
    /// # Arguments
    ///
    /// * `tokens` - The tokens to parse
    /// * `line_info` - Line information for error reporting
    fn new(tokens: &'a [Token], line_info: &'a LineInfo) -> Self {
        Parser {
            tokens,
            current: 0,
            line_info,
            errors: Vec::new(),
        }
    }

    /// Parses the tokens into a list of statements
    ///
    /// # Returns
    ///
    /// The parsed statements or an error message
    fn parse(&mut self) -> CompileResult<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.errors.push(e.to_compiler_error(self.line_info));
                    self.synchronize(); // Skip to next valid statement boundary
                }
            }
        }

        if !self.errors.is_empty() {
            Err(std::mem::take(&mut self.errors))
        } else {
            Ok(statements)
        }
    }

    /// Creates an error at the current token position
    fn error(&self, message: &str) -> ParseError {
        ParseError::new(message, self.peek().pos, self.peek().lexeme.len())
    }

    /// Creates an error at the previous token position
    fn error_previous(&self, message: &str) -> ParseError {
        ParseError::new(message, self.previous().pos, self.previous().lexeme.len())
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        self.try_parse_statement()
    }

    // Skip until a safe synchronization point (e.g., semicolon or statement start)
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Tokentype::Semicolon {
                return;
            }

            match self.peek().token_type {
                Tokentype::Let
                | Tokentype::Fn
                | Tokentype::Struct
                | Tokentype::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }
    /// Parses a single statement
    ///
    /// # Returns
    ///
    /// The parsed statement or an error message
    fn try_parse_statement(&mut self) -> Result<Statement, ParseError> {
        if self.match_token(&Tokentype::Let) {
            self.let_statement()
        } else if self.match_token(&Tokentype::Struct) {
            self.type_definition_statement()
        } else if self.match_token(&Tokentype::Fn) {
            self.function_declaration_statement()
        } else if self.match_token(&Tokentype::Return) {
            self.return_statement()
        } else if self.match_token(&Tokentype::LeftBrace) {
            self.block_statement()
        } else {
            self.expression_statement()
        }
    }

    /// Parses a block statement (a group of statements in braces)
    ///
    /// # Returns
    ///
    /// The parsed block statement or an error message
    fn block_statement(&mut self) -> Result<Statement, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&Tokentype::RightBrace) && !self.is_at_end() {
            statements.push(self.try_parse_statement()?);
        }

        if !self.match_token(&Tokentype::RightBrace) {
            return Err(self.error("Expected '}' after block"));
        }

        Ok(Statement::Block(statements))
    }

    /// Parses a return statement
    ///
    /// # Returns
    ///
    /// The parsed return statement or an error message
    fn return_statement(&mut self) -> Result<Statement, ParseError> {
        let value = if !self.check(&Tokentype::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(self.error("Expected ';' after return value"));
        }

        Ok(Statement::Return(value))
    }

    /// Parses a function declaration
    ///
    /// # Returns
    ///
    /// The parsed function declaration or an error message
    fn function_declaration_statement(&mut self) -> Result<Statement, ParseError> {
        // Parse function name
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error(&format!(
                "Expected function name found {}",
                self.peek().token_type
            )));
        }
        let name = self.advance().lexeme.clone();

        // Parse parameter list
        if !self.match_token(&Tokentype::LeftParen) {
            return Err(self.error(&format!(
                "Expected '(' after function name, found {}",
                self.peek().token_type
            )));
        }

        let mut parameters = Vec::new();
        if !self.check(&Tokentype::RightParen) {
            // Parse first parameter
            parameters.push(self.parameter()?);

            // Parse the rest of the parameters
            while self.match_token(&Tokentype::Comma) {
                if parameters.len() >= 255 {
                    return Err(self.error("Cannot have more than 255 parameters"));
                }
                parameters.push(self.parameter()?);
            }
        }

        if !self.match_token(&Tokentype::RightParen) {
            return Err(self.error(&format!(
                "Expected ')' after parameters found {}",
                self.peek().token_type
            )));
        }

        // Parse return type
        let return_type = if self.match_token(&Tokentype::Arrow) {
            if !self.check(&Tokentype::Identifier) {
                return Err(self.error("Expected return type after '->'"));
            }

            let type_name = self.advance().lexeme.clone();

            if type_name == "int" {
                return Err(self.error("'int' is not a valid type specifier. Use 'i32', 'i64', 'u32', or 'u64' instead"));
            } else if type_name == "float" {
                return Err(
                    self.error("'float' is not a valid type specifier. Use 'f32' or 'f64' instead")
                );
            } else if type_name == "unknown" {
                return Err(self.error("'unknown' is not a valid type specifier"));
            }

            TYPE_REGISTRY.with(|registry| {
                let registry = registry.borrow();
                registry
                    .get_type_by_name(&type_name)
                    .cloned()
                    .unwrap_or_else(unknown_type)
            })
        } else {
            unknown_type()
        };

        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(self.error("Expected '{' before function body"));
        }

        let mut body = Vec::new();
        while !self.check(&Tokentype::RightBrace) && !self.is_at_end() {
            body.push(self.try_parse_statement()?);
        }

        if !self.match_token(&Tokentype::RightBrace) {
            return Err(self.error("Expected '}' after function body"));
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
    fn parameter(&mut self) -> Result<Parameter, ParseError> {
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error("Expected parameter name"));
        }

        let name = self.advance().lexeme.clone();

        if !self.match_token(&Tokentype::Colon) {
            return Err(self.error("Expected ':' after parameter name"));
        }

        if !self.check(&Tokentype::Identifier) {
            return Err(self.error("Expected parameter type"));
        }

        let type_name = self.advance().lexeme.clone();
        let param_type = TYPE_REGISTRY.with(|registry| {
            let registry = registry.borrow();
            registry
                .get_type_by_name(&type_name)
                .cloned()
                .unwrap_or_else(unknown_type)
        });

        if param_type == unknown_type() {
            return Err(self.error(&format!("Unknown type: {}", type_name)));
        }

        Ok(Parameter { name, param_type })
    }

    /// Parses a type definition (struct declaration)
    ///
    /// # Returns
    ///
    /// The parsed type definition or an error message
    fn type_definition_statement(&mut self) -> Result<Statement, ParseError> {
        // Expect struct name
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error("Expected struct name after 'struct' keyword"));
        }

        let name = self.advance().lexeme.clone();

        // Expect opening brace
        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(self.error("Expected '{' after struct name"));
        }

        let mut fields = Vec::new();

        while !self.check(&Tokentype::RightBrace) && !self.is_at_end() {
            if !self.check(&Tokentype::Identifier) {
                return Err(self.error("Expected field name"));
            }
            let field_name = self.advance().lexeme.clone();

            if !self.match_token(&Tokentype::Colon) {
                return Err(self.error("Expected ':' after field name"));
            }

            let field_type = self.parse_type()?;

            fields.push((field_name, field_type));

            if !self.match_token(&Tokentype::Comma) && !self.check(&Tokentype::RightBrace) {
                return Err(self.error("Expected ',' after field or '}'"));
            }
        }

        if !self.match_token(&Tokentype::RightBrace) {
            return Err(self.error("Expected '}' after struct fields"));
        }

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(self.error("Expected ';' after struct definition"));
        }

        Ok(Statement::TypeDefinition(TypeDefinitionStmt {
            name,
            fields,
        }))
    }

    /// Parses a variable declaration
    ///
    /// # Returns
    ///
    /// The parsed variable declaration or an error message
    fn let_statement(&mut self) -> Result<Statement, ParseError> {
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error("Expected identifier after 'let'"));
        }

        let token = self.advance();
        let name = token.lexeme.clone();
        let mut var_type = unknown_type();

        if self.match_token(&Tokentype::Colon) {
            if !self.check(&Tokentype::Identifier) {
                return Err(self.error("Expected type name after colon"));
            }

            let type_name = self.advance().lexeme.clone();

            // Explicitly reject placeholder types as type specifiers
            if type_name == "int" {
                return Err(self.error("'int' is not a valid type specifier. Use 'i32', 'i64', 'u32', or 'u64' instead"));
            } else if type_name == "float" {
                return Err(
                    self.error("'float' is not a valid type specifier. Use 'f32' or 'f64' instead")
                );
            }

            var_type = TYPE_REGISTRY.with(|registry| {
                let registry = registry.borrow();
                registry
                    .get_type_by_name(&type_name)
                    .cloned()
                    .unwrap_or_else(unknown_type)
            });

            if var_type == unknown_type() {
                return Err(self.error(&format!("Unknown type: {}", type_name)));
            }
        }

        if !self.match_token(&Tokentype::Equal) {
            return Err(self.error("Expected '=' after variable name"));
        }

        let expr = self.expression()?;

        // Expect semicolon
        if !self.match_token(&Tokentype::Semicolon) {
            return Err(self.error("Expected ';' after let statement"));
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
    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expression()?;

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(self.error("Expected ';' after expression"));
        }

        Ok(Statement::Expression(expr))
    }

    /// Parses an expression
    ///
    /// # Returns
    ///
    /// The parsed expression or an error message
    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.logical_or()
    }

    /// Parses a logical OR expression
    ///
    /// # Returns
    ///
    /// The parsed logical OR expression or an error message
    fn logical_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.logical_and()?;

        while self.match_token(&Tokentype::Or) {
            let right = self.logical_and()?;
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
                expr_type: bool_type(),
            });
        }

        Ok(expr)
    }

    /// Parses a logical AND expression
    ///
    /// # Returns
    ///
    /// The parsed logical AND expression or an error message
    fn logical_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token(&Tokentype::And) {
            let right = self.equality()?;
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
                expr_type: bool_type(),
            });
        }

        Ok(expr)
    }

    /// Parses an equality expression (== and !=)
    ///
    /// # Returns
    ///
    /// The parsed equality expression or an error message
    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_any(&[Tokentype::EqualEqual, Tokentype::NotEqual]) {
            let operator =  match self.previous().token_type {
                Tokentype::EqualEqual => BinaryOperator::Equal,
                Tokentype::NotEqual => BinaryOperator::NotEqual,
                _ => unreachable!(),
                
            };
            let right = self.comparison()?;
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: bool_type(),
            });
        }

        Ok(expr)
    }

    /// Parses a comparison expression (>, <, >=, <=)
    ///
    /// # Returns
    ///
    /// The parsed comparison expression or an error message
    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;

        while self.match_any(&[
            Tokentype::Greater,
            Tokentype::GreaterEqual,
            Tokentype::Less,
            Tokentype::LessEqual,
        ]) {
            let operator = match self.previous().token_type {
                Tokentype::Greater => BinaryOperator::GreaterThan,
                Tokentype::GreaterEqual => BinaryOperator::GreaterThanOrEqual,
                Tokentype::Less => BinaryOperator::LessThan,
                Tokentype::LessEqual => BinaryOperator::LessThanOrEqual,
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: bool_type(),
            });
        }

        Ok(expr)
    }

    /// Parses a term (addition/subtraction)
    ///
    /// # Returns
    ///
    /// The parsed term or an error message
    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;

        while self.match_any(&[Tokentype::Plus, Tokentype::Minus]) {
        let operator = match self.previous().token_type {
                Tokentype::Plus => BinaryOperator::Add,
                Tokentype::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
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
    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;

        while self.match_any(&[Tokentype::Multiply, Tokentype::Divide]) {
            let operator = match self.previous().token_type {
                Tokentype::Multiply => BinaryOperator::Multiply,
                Tokentype::Divide => BinaryOperator::Divide,
                _ => unreachable!(),
            };
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
    fn unary(&mut self) -> Result<Expression, ParseError> {
        if self.match_token(&Tokentype::Minus) {
            let right = self.primary()?;
            return Ok(Expression::Unary(UnaryExpr {
                operator: UnaryOperator::Negate,
                right: Box::new(right),
                expr_type: unknown_type(),
            }));
        }

        if self.match_token(&Tokentype::Not) {
            let right = self.primary()?;
            return Ok(Expression::Unary(UnaryExpr {
                operator: UnaryOperator::Not,
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
    fn primary(&mut self) -> Result<Expression, ParseError> {
        if self.match_token(&Tokentype::IntegerLiteral) {
            return self.parse_integer();
        }

        if self.match_token(&Tokentype::FloatLiteral) {
            return self.parse_float();
        }

        if self.match_token(&Tokentype::StringLiteral) {
            let value = self.previous().lexeme.clone();
            return Ok(Expression::Literal(LiteralExpr {
                value: LiteralValue::String(value),
                expr_type: string_type(),
            }));
        }

        if self.match_token(&Tokentype::BooleanLiteral) {
            let lexeme = self.previous().lexeme.clone();
            let bool_value = lexeme == "true";
            return Ok(Expression::Literal(LiteralExpr {
                value: LiteralValue::Boolean(bool_value),
                expr_type: bool_type(),
            }));
        }

        if self.match_token(&Tokentype::LeftParen) {
            let expr = self.expression()?;
            if !self.match_token(&Tokentype::RightParen) {
                return Err(self.error("Expected ')' after expression"));
            }
            return Ok(expr);
        }

        if self.match_token(&Tokentype::Identifier) {
            let name = self.previous().lexeme.clone();

            if self.match_token(&Tokentype::LeftParen) {
                return self.finish_call(name);
            }

            return Ok(Expression::Variable(name));
        }

        Err(self.error(&format!("Expected expression, found {}", self.peek())))
    }

    /// Parses a float literal with optional type suffix
    ///
    /// # Returns
    ///
    /// The parsed float literal expression or an error message
    fn parse_float(&mut self) -> Result<Expression, ParseError> {
        let value_str = self.previous().lexeme.clone();
        let value = value_str
            .parse::<f64>()
            .map_err(|_| self.error_previous(&format!("Invalid float: {}", value_str)))?;

        if self.check(&Tokentype::Identifier) {
            let type_name = self.peek().lexeme.clone();

            match type_name.as_str() {
                "f32" => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F32(value as f32),
                        expr_type: f32_type(),
                    }));
                }
                "f64" => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F64(value),
                        expr_type: f64_type(),
                    }));
                }
                _ => {}
            }
        }

        // Unspecified float literal
        Ok(Expression::Literal(LiteralExpr {
            value: LiteralValue::UnspecifiedFloat(value),
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
    fn finish_call(&mut self, name: String) -> Result<Expression, ParseError> {
        let mut arguments = Vec::new();

        if !self.check(&Tokentype::RightParen) {
            // Parse first argument
            arguments.push(self.expression()?);

            // Parse rest of the arguments
            while self.match_token(&Tokentype::Comma) {
                if arguments.len() >= 255 {
                    return Err(self.error("Cannot have more than 255 arguments"));
                }
                arguments.push(self.expression()?);
            }
        }

        if !self.match_token(&Tokentype::RightParen) {
            return Err(self.error("Expected ')' after function arguments"));
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
    fn parse_integer(&mut self) -> Result<Expression, ParseError> {
        let value_str = self.previous().lexeme.clone();
        let base_value = value_str
            .parse::<i64>()
            .map_err(|_| self.error_previous(&format!("Invalid integer: {}", value_str)))?;

        if self.check(&Tokentype::Identifier) {
            let type_name = self.peek().lexeme.clone();

            match type_name.as_str() {
                "i32" => {
                    self.advance();
                    if base_value > i32::MAX as i64 || base_value < i32::MIN as i64 {
                        return Err(self.error_previous(&format!(
                            "Value {} is out of range for i32",
                            base_value
                        )));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::I32(base_value as i32),
                        expr_type: i32_type(),
                    }));
                }
                "i64" => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::I64(base_value),
                        expr_type: i64_type(),
                    }));
                }
                "u32" => {
                    self.advance();
                    if base_value < 0 || base_value > u32::MAX as i64 {
                        return Err(self.error_previous(&format!(
                            "Value {} is out of range for u32",
                            base_value
                        )));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::U32(base_value as u32),
                        expr_type: u32_type(),
                    }));
                }
                "u64" => {
                    self.advance();
                    if base_value < 0 {
                        return Err(self.error_previous(&format!(
                            "Value {} is out of range for u64",
                            base_value
                        )));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::U64(base_value as u64),
                        expr_type: u64_type(),
                    }));
                }
                "f32" => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F32(base_value as f32),
                        expr_type: f32_type(),
                    }));
                }
                "f64" => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F64(base_value as f64),
                        expr_type: f64_type(),
                    }));
                }
                _ => {}
            }
        }

        // Unspecified integer literal
        Ok(Expression::Literal(LiteralExpr {
            value: LiteralValue::UnspecifiedInteger(base_value),
            expr_type: unspecified_int_type(),
        }))
    }

    /// Parses a type name
    ///
    /// # Returns
    ///
    /// The type ID for the parsed type or an error
    fn parse_type(&mut self) -> Result<TypeId, ParseError> {
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error("Expected type identifier"));
        }

        let type_name = self.advance().lexeme.clone();

        // Check for placeholder types
        if type_name == "int" {
            return Err(self.error(
                "'int' is not a valid type specifier. Use 'i32', 'i64', 'u32', or 'u64' instead",
            ));
        } else if type_name == "float" {
            return Err(
                self.error("'float' is not a valid type specifier. Use 'f32' or 'f64' instead")
            );
        } else if type_name == "unknown" {
            return Err(self.error("'unknown' is not a valid type specifier"));
        }

        TYPE_REGISTRY.with(|registry| {
            let registry = registry.borrow();
            if let Some(type_id) = registry.get_type_by_name(&type_name) {
                Ok(type_id.clone())
            } else {
                Err(self.error(&format!("Unknown type: {}", type_name)))
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
    fn match_token(&mut self, token_type: &Tokentype) -> bool {
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
        for token_type in types.iter() {
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
    fn check(&self, token_type: &Tokentype) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    /// Advances to the next token and returns the previous token
    ///
    /// # Returns
    ///
    /// The token that was current before advancing, if the end of the token stream was not reached
    /// Otherwise, returns the last token
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
    /// true if all tokens have been procesed, false otherwise
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
