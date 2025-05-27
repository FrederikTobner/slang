use crate::error::LineInfo;
use crate::error::{CompileResult, CompilerError};
use crate::error_codes::ErrorCode;
use crate::token::{Token, Tokentype};
use slang_shared::{CompilationContext, SymbolKind};
use slang_ir::ast::{
    BinaryExpr, BinaryOperator, ConditionalExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt,
    IfStatement, LetStatement, LiteralExpr, LiteralValue, Parameter, Statement, TypeDefinitionStmt, UnaryExpr,
    UnaryOperator,
};
use slang_ir::SourceLocation;
use slang_types::{
    PrimitiveType, TypeId, TYPE_NAME_F32, TYPE_NAME_F64, TYPE_NAME_FLOAT, TYPE_NAME_I32, TYPE_NAME_I64, TYPE_NAME_INT, TYPE_NAME_U32, TYPE_NAME_U64, TYPE_NAME_UNKNOWN
};

/// Error that occurs during parsing
#[derive(Debug)]
pub struct ParseError {
    /// The structured error code for this error
    error_code: ErrorCode,
    /// Error message describing the problem
    message: String,
    /// Position in the source code where the error occurred
    position: usize,
    /// Length of the underlined part
    underline_length: usize,
}

impl ParseError {
    /// Creates a new parse error with the given error code, message and position
    pub fn new(error_code: ErrorCode, message: &str, position: usize, underline_length: usize) -> Self {
        ParseError {
            error_code,
            message: message.to_string(),
            position,
            underline_length,
        }
    }

    pub fn to_compiler_error(&self, line_info: &LineInfo) -> CompilerError {
        let line_pos = line_info.get_line_col(self.position);
        CompilerError::new(
            self.error_code,
            self.message.clone(),
            line_pos.0,
            line_pos.1,
            self.position,
            Some(self.underline_length),
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
    /// Compilation context for type information
    context: &'a mut CompilationContext, 
}

pub fn parse<'a>(
    tokens: &'a [Token],
    line_info: &'a LineInfo,
    context: &'a mut CompilationContext,
) -> CompileResult<Vec<Statement>> {
    let mut parser = Parser::new(tokens, line_info, context);
    parser.parse()
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given tokens and line information
    ///
    /// ### Arguments
    ///
    /// * `tokens` - The tokens to parse
    /// * `line_info` - Line information for error reporting
    /// * `context` - The compilation context
    fn new(
        tokens: &'a [Token],
        line_info: &'a LineInfo,
        context: &'a mut CompilationContext,
    ) -> Self {
        Parser {
            tokens,
            current: 0,
            line_info,
            errors: Vec::new(),
            context,
        }
    }

    /// Parses the tokens into a list of statements
    ///
    /// ### Returns
    ///
    /// The parsed statements or an error message
    fn parse(&mut self) -> CompileResult<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.errors.push(e.to_compiler_error(self.line_info));
                    self.synchronize(); 
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
    /// 
    /// ### Arguments
    /// 
    /// * `èrror_code` - The error code for the error
    /// * `message` - The error message to display
    /// 
    /// ### Returns
    /// A new ParseError with the current token position and length
    fn error(&self, error_code: ErrorCode, message: &str) -> ParseError {
        ParseError::new(error_code, message, self.peek().pos, self.peek().lexeme.len())
    }

    /// Creates an error at the previous token position
    /// 
    /// ### Arguments
    /// 
    /// * `error_code` - The error code for the error
    /// * `message` - The error message to display
    /// 
    /// ### Returns
    /// A new ParseError with the previous token position and length
    fn error_previous(&self, error_code: ErrorCode, message: &str) -> ParseError {
        ParseError::new(error_code, message, self.previous().pos, self.previous().lexeme.len())
    }

    /// Skip until a safe synchronization point (e.g., semicolon or statement start)
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Tokentype::Semicolon {
                return;
            }

            match self.peek().token_type {
                Tokentype::Let | Tokentype::Fn | Tokentype::Struct | Tokentype::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }
    /// Parses a single statement
    ///
    /// ### Returns
    ///
    /// The parsed statement or an error message
    fn statement(&mut self) -> Result<Statement, ParseError> {
        if self.match_token(&Tokentype::Let) {
            self.let_statement()
        } else if self.match_token(&Tokentype::Struct) {
            self.type_definition_statement()
        } else if self.match_token(&Tokentype::Fn) {
            self.function_declaration_statement()
        } else if self.match_token(&Tokentype::Return) {
            self.return_statement()
        } else if self.match_token(&Tokentype::If) {
            self.if_statement()
        } else if self.match_token(&Tokentype::LeftBrace) {
            self.block_statement()
        } else if self.check(&Tokentype::Identifier) && self.check_next(&Tokentype::Equal) {
            self.assignment_statement()
        } else {
            self.expression_statement()
        }
    }

    /// Parses a block statement (a group of statements in braces)
    ///
    /// ### Returns
    ///
    /// The parsed block statement or an error message
    fn block_statement(&mut self) -> Result<Statement, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&Tokentype::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        if !self.match_token(&Tokentype::RightBrace) {
            return Err(self.error(ErrorCode::ExpectedClosingBrace, "Expected '}' after block"));
        }

        Ok(Statement::Block(statements))
    }

    /// Parses a return statement
    ///
    /// ### Returns
    ///
    /// The parsed return statement or an error message
    fn return_statement(&mut self) -> Result<Statement, ParseError> {
        let value = if !self.check(&Tokentype::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(self.error(ErrorCode::ExpectedSemicolon, "Expected ';' after return value"));
        }

        Ok(Statement::Return(value))
    }

    /// Parses a function declaration
    ///
    /// ### Returns
    ///
    /// The parsed function declaration or an error message
    fn function_declaration_statement(&mut self) -> Result<Statement, ParseError> {
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error(ErrorCode::ExpectedIdentifier, &format!(
                "Expected function name found {}",
                self.peek().token_type
            )));
        }
        let token = self.advance();
        let token_pos = token.pos;
        let name = token.lexeme.clone();

        let (line, column) = self.line_info.get_line_col(token_pos);
        let location =
            slang_ir::source_location::SourceLocation::new(token_pos, line, column, name.len());

        if !self.match_token(&Tokentype::LeftParen) {
            return Err(self.error(ErrorCode::ExpectedOpeningParen, &format!(
                "Expected '(' after function name, found {}",
                self.peek().token_type
            )));
        }

        let mut parameters = Vec::new();
        if !self.check(&Tokentype::RightParen) {
            parameters.push(self.parameter()?);
            while self.match_token(&Tokentype::Comma) {
                if parameters.len() >= 255 {
                    return Err(self.error(ErrorCode::InvalidSyntax, "Cannot have more than 255 parameters"));
                }
                parameters.push(self.parameter()?);
            }
        }

        if !self.match_token(&Tokentype::RightParen) {
            return Err(self.error(ErrorCode::ExpectedClosingParen, &format!(
                "Expected ')' after parameters found {}",
                self.peek().token_type
            )));
        }

        let return_type = if self.match_token(&Tokentype::Arrow) {
            if !self.check(&Tokentype::Identifier) {
                return Err(self.error(ErrorCode::ExpectedType, "Expected return type after '->'"));
            }

            let type_name_token = self.advance();
            let type_name = type_name_token.lexeme.clone();
            let token_pos = type_name_token.pos;
            let token_len = type_name_token.lexeme.len();

            if type_name == TYPE_NAME_INT {
                return Err(self.error_previous(ErrorCode::InvalidSyntax, &format!(
                    "'{}' is not a valid type specifier. Use '{}', '{}', '{}', or '{}' instead",
                    TYPE_NAME_INT, TYPE_NAME_I32, TYPE_NAME_I64, TYPE_NAME_U32, TYPE_NAME_U64
                )));
            } else if type_name == TYPE_NAME_FLOAT {
                return Err(self.error_previous(ErrorCode::InvalidSyntax, &format!(
                    "'{}' is not a valid type specifier. Use '{}' or '{}' instead",
                    TYPE_NAME_FLOAT, TYPE_NAME_F32, TYPE_NAME_F64
                )));
            } else if type_name == TYPE_NAME_UNKNOWN {
                return Err(self.error_previous(ErrorCode::InvalidSyntax, &format!(
                    "'{}' is not a valid type specifier",
                    TYPE_NAME_UNKNOWN
                )));
            }

            let type_id_option = self.context.lookup_symbol(&type_name).and_then(|symbol| {
                if symbol.kind == SymbolKind::Type {
                    Some(symbol.type_id.clone())
                } else {
                    None
                }
            });

            match type_id_option {
                Some(type_id) => type_id,
                None => {
                    let error = ParseError::new(
                        ErrorCode::UnknownType,
                        &format!("Unknown type name: {}", type_name),
                        token_pos,
                        token_len,
                    )
                    .to_compiler_error(self.line_info);
                    self.errors.push(error);
                    TypeId(PrimitiveType::Unknown as usize) // Return unknown_type on error to allow parsing to continue
                }
            }
        } else {
            TypeId(PrimitiveType::Unknown as usize) // TODO: Introduce a void or unit type
        };

        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(self.error(ErrorCode::ExpectedOpeningBrace, "Expected '{' before function body"));
        }

        let mut body = Vec::new();
        while !self.check(&Tokentype::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }

        if !self.match_token(&Tokentype::RightBrace) {
            return Err(self.error(ErrorCode::ExpectedClosingBrace, "Expected '}' after function body"));
        }

        Ok(Statement::FunctionDeclaration(FunctionDeclarationStmt {
            name,
            parameters,
            return_type,
            body,
            location,
        }))
    }

    /// Parses a function parameter
    ///
    /// ### Returns
    ///
    /// The parsed parameter or an error message
    fn parameter(&mut self) -> Result<Parameter, ParseError> {
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error(ErrorCode::ExpectedIdentifier, "Expected parameter name"));
        }

        let token_pos = self.peek().pos;
        let token = self.advance();
        let name = token.lexeme.clone();

        let (line, column) = self.line_info.get_line_col(token_pos);
        let location = SourceLocation::new(token_pos, line, column, name.len());

        if !self.match_token(&Tokentype::Colon) {
            return Err(self.error(ErrorCode::ExpectedColon, "Expected ':' after parameter name"));
        }

        if !self.check(&Tokentype::Identifier) {
            return Err(self.error(ErrorCode::ExpectedType, "Expected parameter type"));
        }

        let type_name_token = self.advance();
        let type_name = type_name_token.lexeme.clone();
        let token_pos = type_name_token.pos;
        let token_len = type_name_token.lexeme.len();

        let unknown_type = TypeId(PrimitiveType::Unknown as usize);
        let param_type_option = self.context.lookup_symbol(&type_name).and_then(|symbol| {
            if symbol.kind == SymbolKind::Type {
                Some(symbol.type_id.clone())
            } else {
                None
            }
        });

        let param_type = match param_type_option {
            Some(type_id) => type_id,
            None => {
                let error = ParseError::new(
                    ErrorCode::UnknownType,
                    &format!("Unknown type name: {}", type_name),
                    token_pos,
                    token_len,
                )
                .to_compiler_error(self.line_info);
                self.errors.push(error);
                unknown_type.clone() 
            }
        };

        Ok(Parameter {
            name,
            param_type,
            location,
        })
    }

    /// Parses a type definition (struct declaration)
    ///
    /// ### Returns
    ///
    /// The parsed type definition or an error message
    fn type_definition_statement(&mut self) -> Result<Statement, ParseError> {
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error(ErrorCode::ExpectedIdentifier, "Expected struct name after 'struct' keyword"));
        }

        let token = self.peek();
        let location = self.source_location_from_token(token);
        let name = self.advance().lexeme.clone();

        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(self.error(ErrorCode::ExpectedOpeningBrace, "Expected '{' after struct name"));
        }

        let mut fields = Vec::new();

        while !self.check(&Tokentype::RightBrace) && !self.is_at_end() {
            if !self.check(&Tokentype::Identifier) {
                return Err(self.error(ErrorCode::ExpectedIdentifier, "Expected field name"));
            }
            let field_name = self.advance().lexeme.clone();

            if !self.match_token(&Tokentype::Colon) {
                return Err(self.error(ErrorCode::ExpectedColon, "Expected ':' after field name"));
            }

            let field_type = self.parse_type()?;

            fields.push((field_name, field_type));

            if !self.match_token(&Tokentype::Comma) && !self.check(&Tokentype::RightBrace) {
                return Err(self.error(ErrorCode::ExpectedComma, "Expected ',' after field or '}'"));
            }
        }

        if !self.match_token(&Tokentype::RightBrace) {
            return Err(self.error(ErrorCode::ExpectedClosingBrace, "Expected '}' after struct fields"));
        }

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(self.error(ErrorCode::ExpectedSemicolon, "Expected ';' after struct definition"));
        }

        Ok(Statement::TypeDefinition(TypeDefinitionStmt {
            name,
            fields,
            location,
        }))
    }

    /// Parses a variable declaration
    ///
    /// ### Returns
    ///
    /// The parsed variable declaration or an error message
    fn let_statement(&mut self) -> Result<Statement, ParseError> {
        let is_mutable = self.match_token(&Tokentype::Mut);
        
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error(ErrorCode::ExpectedIdentifier, "Expected identifier after 'let'"));
        }

        let token_pos = self.peek().pos;
        let (line, column) = self.line_info.get_line_col(token_pos);

        let token = self.advance();
        let name = token.lexeme.clone();
        let location =
            slang_ir::source_location::SourceLocation::new(token_pos, line, column, name.len());
        let mut var_type = TypeId(PrimitiveType::Unknown as usize); 

        if self.match_token(&Tokentype::Colon) {
            if !self.check(&Tokentype::Identifier) {
                return Err(self.error(ErrorCode::ExpectedType, "Expected type name after colon"));
            }
            let type_name_token = self.advance();
            let type_name = type_name_token.lexeme.clone();
            let token_pos = type_name_token.pos;
            let token_len = type_name_token.lexeme.len();

            if type_name == TYPE_NAME_INT {
                return Err(self.error_previous(ErrorCode::InvalidSyntax, &format!(
                    "'{}' is not a valid type specifier. Use '{}', '{}', '{}', or '{}' instead",
                    TYPE_NAME_INT, TYPE_NAME_I32, TYPE_NAME_I64, TYPE_NAME_U32, TYPE_NAME_U64
                )));
            } else if type_name == TYPE_NAME_FLOAT {
                return Err(self.error_previous(ErrorCode::InvalidSyntax, &format!(
                    "'{}' is not a valid type specifier. Use '{}' or '{}' instead",
                    TYPE_NAME_FLOAT, TYPE_NAME_F32, TYPE_NAME_F64
                )));
            }

            let unknown_type = TypeId(PrimitiveType::Unknown as usize);
            let var_type_option = self.context.lookup_symbol(&type_name).and_then(|symbol| {
                if symbol.kind == SymbolKind::Type {
                    Some(symbol.type_id.clone())
                } else {
                    None
                }
            });

            var_type = match var_type_option {
                Some(type_id) => type_id,
                None => {
                    let error = ParseError::new(
                        ErrorCode::InvalidSyntax,
                        &format!("Unknown type name: {}", type_name),
                        token_pos,
                        token_len,
                    )
                    .to_compiler_error(self.line_info);
                    self.errors.push(error);
                    unknown_type.clone() 
                }
            };

            if var_type == unknown_type
                && !self.errors.iter().any(|e| {
                    e.message
                        .contains(&format!("Unknown type name: {}", type_name))
                })
            {
                return Err(self.error_previous(ErrorCode::InvalidSyntax, &format!("Unknown type: {}", type_name)));
            }
        }

        if !self.match_token(&Tokentype::Equal) {
            return Err(self.error(ErrorCode::ExpectedEquals, "Expected '=' after variable name"));
        }

        let expr = self.expression()?;

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(self.error(ErrorCode::ExpectedSemicolon, "Expected ';' after let statement"));
        }

        Ok(Statement::Let(LetStatement {
            name,
            is_mutable,
            value: expr,
            expr_type: var_type,
            location,
        }))
    }

    /// Parses an expression statement
    ///
    /// ### Returns
    ///
    /// The parsed expression statement or an error message
    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expression()?;

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(self.error(ErrorCode::ExpectedSemicolon, "Expected ';' after expression"));
        }

        Ok(Statement::Expression(expr))
    }

    /// Parses an expression
    ///
    /// ### Returns
    ///
    /// The parsed expression or an error message
    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.logical_or()
    }

    /// Parses a logical OR expression
    ///
    /// ### Returns
    ///
    /// The parsed logical OR expression or an error message
    fn logical_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.logical_and()?;

        while self.match_token(&Tokentype::Or) {
            let left_location = expr.location();
            let right = self.logical_and()?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);
            
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
                expr_type: TypeId(PrimitiveType::Bool as usize),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses a logical AND expression
    ///
    /// ### Returns
    ///
    /// The parsed logical AND expression or an error message
    fn logical_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token(&Tokentype::And) {
            let left_location = expr.location();
            let right = self.equality()?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);
            
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
                expr_type: TypeId(PrimitiveType::Bool as usize),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses an equality expression (== and !=)
    ///
    /// ### Returns
    ///
    /// The parsed equality expression or an error message
    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_any(&[Tokentype::EqualEqual, Tokentype::NotEqual]) {
            let left_location = expr.location();
            let token = self.previous();
            let operator = match token.token_type {
                Tokentype::EqualEqual => BinaryOperator::Equal,
                Tokentype::NotEqual => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);
            
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: TypeId(PrimitiveType::Bool as usize),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses a comparison expression (>, <, >=, <=)
    ///
    /// ### Returns
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
            let left_location = expr.location();
            let token = self.previous();
            let operator = match token.token_type {
                Tokentype::Greater => BinaryOperator::GreaterThan,
                Tokentype::GreaterEqual => BinaryOperator::GreaterThanOrEqual,
                Tokentype::Less => BinaryOperator::LessThan,
                Tokentype::LessEqual => BinaryOperator::LessThanOrEqual,
                _ => unreachable!(),
            };
            let right = self.term()?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);
            
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: TypeId(PrimitiveType::Bool as usize),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses a term (addition/subtraction)
    ///
    /// ### Returns
    ///
    /// The parsed term or an error message
    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;

        while self.match_any(&[Tokentype::Plus, Tokentype::Minus]) {
            let left_location = expr.location();
            let token = self.previous();
            let operator = match token.token_type {
                Tokentype::Plus => BinaryOperator::Add,
                Tokentype::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);
            
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: TypeId(PrimitiveType::Unknown as usize),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses a factor (multiplication/division)
    ///
    /// ### Returns
    ///
    /// The parsed factor or an error message
    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;

        while self.match_any(&[Tokentype::Multiply, Tokentype::Divide]) {
            let left_location = expr.location();
            let token = self.previous();
            let operator = match token.token_type {
                Tokentype::Multiply => BinaryOperator::Multiply,
                Tokentype::Divide => BinaryOperator::Divide,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            let right_location = right.location();
            let span_location = left_location.span_to(&right_location);
            
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: TypeId(PrimitiveType::Unknown as usize),
                location: span_location,
            });
        }

        Ok(expr)
    }

    /// Parses a unary expression
    ///
    /// ### Returns
    ///
    /// The parsed unary expression or an error message
    fn unary(&mut self) -> Result<Expression, ParseError> {
        if self.match_token(&Tokentype::Minus) {
            let token = self.previous();
            let operator_location = self.source_location_from_token(token);
            let right = self.primary()?;
            let right_location = right.location();
            let span_location = operator_location.span_to(&right_location);
            
            return Ok(Expression::Unary(UnaryExpr {
                operator: UnaryOperator::Negate,
                right: Box::new(right),
                expr_type: TypeId(PrimitiveType::Unknown as usize),
                location: span_location,
            }));
        }

        if self.match_token(&Tokentype::Not) {
            let token = self.previous();
            let operator_location = self.source_location_from_token(token);
            let right = self.primary()?;
            let right_location = right.location();
            let span_location = operator_location.span_to(&right_location);
            
            return Ok(Expression::Unary(UnaryExpr {
                operator: UnaryOperator::Not,
                right: Box::new(right),
                expr_type: TypeId(PrimitiveType::Bool as usize),
                location: span_location,
            }));
        }

        self.primary()
    }

    /// Parses a primary expression (literal, variable, or grouped expression)
    ///
    /// ### Returns
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
            let token = self.previous();
            let value = token.lexeme.clone();
            return Ok(Expression::Literal(LiteralExpr {
                value: LiteralValue::String(value),
                expr_type: TypeId(PrimitiveType::String as usize),
                location: self.source_location_from_token(token),
            }));
        }

        if self.match_token(&Tokentype::BooleanLiteral) {
            let token = self.previous();
            let lexeme = token.lexeme.clone();
            let bool_value = lexeme == "true";
            return Ok(Expression::Literal(LiteralExpr {
                value: LiteralValue::Boolean(bool_value),
                expr_type: TypeId(PrimitiveType::Bool as usize),
                location: self.source_location_from_token(token),
            }));
        }

        if self.match_token(&Tokentype::If) {
            return self.conditional_expression();
        }

        if self.match_token(&Tokentype::LeftParen) {
            let expr = self.expression()?;
            if !self.match_token(&Tokentype::RightParen) {
                return Err(self.error(ErrorCode::ExpectedClosingParen, "Expected ')' after expression"));
            }
            return Ok(expr);
        }

        if self.match_token(&Tokentype::Identifier) {
            let name = self.previous().lexeme.clone();

            if self.match_token(&Tokentype::LeftParen) {
                return self.finish_call(name);
            }

            let token = self.previous();
            let location = self.source_location_from_token(token);
            return Ok(Expression::Variable(name, location));
        }

        Err(self.error(ErrorCode::ExpectedExpression, &format!("Expected expression, found {}", self.peek())))
    }

    /// Parses a float literal with optional type suffix
    ///
    /// ### Returns
    ///
    /// The parsed float literal expression or an error message
    fn parse_float(&mut self) -> Result<Expression, ParseError> {
        let token = self.previous();
        let value_str = token.lexeme.clone();
        let location = self.source_location_from_token(token);
        let value = value_str
            .parse::<f64>()
            .map_err(|_| self.error_previous(ErrorCode::InvalidNumberLiteral, &format!("Invalid float: {}", value_str)))?;

        if self.check(&Tokentype::Identifier) {
            let type_name = self.peek().lexeme.clone();

            match type_name.as_str() {
                TYPE_NAME_F32 => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F32(value as f32),
                        expr_type: TypeId(PrimitiveType::F32 as usize),
                        location,
                    }));
                }
                TYPE_NAME_F64 => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F64(value),
                        expr_type: TypeId(PrimitiveType::F64 as usize),
                        location,
                    }));
                }
                _ => {}
            }
        }

        Ok(Expression::Literal(LiteralExpr {
            value: LiteralValue::UnspecifiedFloat(value),
            expr_type: TypeId(PrimitiveType::UnspecifiedFloat as usize) ,
            location,
        }))
    }

    /// Finishes parsing a function call after the name and '('
    ///
    /// #### Arguments
    ///
    /// * `name` - The name of the function being called
    ///
    /// ### Returns
    ///
    /// The parsed function call expression or an error message
    fn finish_call(&mut self, name: String) -> Result<Expression, ParseError> {
        let name_token = self.previous();
        let start_location = self.source_location_from_token(name_token);

        let mut arguments = Vec::new();

        if !self.check(&Tokentype::RightParen) {
            arguments.push(self.expression()?);

            while self.match_token(&Tokentype::Comma) {
                if arguments.len() >= 255 {
                    return Err(self.error(ErrorCode::InvalidSyntax, "Cannot have more than 255 arguments"));
                }
                arguments.push(self.expression()?);
            }
        }

        if !self.match_token(&Tokentype::RightParen) {
            return Err(self.error(ErrorCode::ExpectedClosingParen, "Expected ')' after function arguments"));
        }

        let closing_paren_token = self.previous();
        let end_location = self.source_location_from_token(closing_paren_token);
        let span_location = start_location.span_to(&end_location);

        Ok(Expression::Call(FunctionCallExpr {
            name,
            arguments,
            expr_type: TypeId(PrimitiveType::Unknown as usize),
            location: span_location,
        }))
    }

    /// Parses an integer literal with optional type suffix
    ///
    /// ### Returns
    ///
    /// The parsed integer literal expression or an error message
    fn parse_integer(&mut self) -> Result<Expression, ParseError> {
        let token = self.previous();
        let value_str = token.lexeme.clone();
        let base_value = value_str
            .parse::<i64>()
            .map_err(|_| self.error_previous(ErrorCode::InvalidNumberLiteral, &format!("Invalid integer: {}", value_str)))?;
        let location = self.source_location_from_token(token);

        if self.check(&Tokentype::Identifier) {
            let type_name = self.peek().lexeme.clone();

            match type_name.as_str() {
                TYPE_NAME_I32 => {
                    self.advance();
                    if base_value > i32::MAX as i64 || base_value < i32::MIN as i64 {
                        return Err(self.error_previous(ErrorCode::ValueOutOfRange, &format!(
                            "Value {} is out of range for {}",
                            base_value, TYPE_NAME_I32
                        )));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::I32(base_value as i32),
                        expr_type: TypeId(PrimitiveType::I32 as usize),
                        location,
                    }));
                }
                TYPE_NAME_I64 => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::I64(base_value),
                        expr_type: TypeId(PrimitiveType::I64 as usize),
                        location,
                    }));
                }
                TYPE_NAME_U32 => {
                    self.advance();
                    if base_value < 0 || base_value > u32::MAX as i64 {
                        return Err(self.error_previous(ErrorCode::ValueOutOfRange, &format!(
                            "Value {} is out of range for {}",
                            base_value, TYPE_NAME_U32
                        )));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::U32(base_value as u32),
                        expr_type: TypeId(PrimitiveType::U32 as usize),
                        location,
                    }));
                }
                TYPE_NAME_U64 => {
                    self.advance();
                    if base_value < 0 {
                        return Err(self.error_previous(ErrorCode::ValueOutOfRange, &format!(
                            "Value {} is out of range for {}",
                            base_value, TYPE_NAME_U64
                        )));
                    }
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::U64(base_value as u64),
                        expr_type: TypeId(PrimitiveType::U64 as usize),
                        location,
                    }));
                }
                TYPE_NAME_F32 => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F32(base_value as f32),
                        expr_type: TypeId(PrimitiveType::F32 as usize),
                        location,
                    }));
                }
                TYPE_NAME_F64 => {
                    self.advance();
                    return Ok(Expression::Literal(LiteralExpr {
                        value: LiteralValue::F64(base_value as f64),
                        expr_type: TypeId(PrimitiveType::F64 as usize),
                        location,
                    }));
                }
                _ => {}
            }
        }

        Ok(Expression::Literal(LiteralExpr {
            value: LiteralValue::UnspecifiedInteger(base_value),
            expr_type: TypeId(PrimitiveType::UnspecifiedInt as usize) ,
            location,
        }))
    }

    /// Parses a type name
    ///
    /// ### Returns
    ///
    /// The type ID for the parsed type or an error
    fn parse_type(&mut self) -> Result<TypeId, ParseError> {
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error(ErrorCode::ExpectedIdentifier, "Expected type identifier"));
        }

        let type_name_token = self.advance();
        let type_name = type_name_token.lexeme.clone();

        if type_name == TYPE_NAME_INT {
            return Err(self.error(ErrorCode::UnknownType, &format!(
                "'{}' is not a valid type specifier. Use '{}', '{}', '{}', or '{}' instead",
                TYPE_NAME_INT, TYPE_NAME_I32, TYPE_NAME_I64, TYPE_NAME_U32, TYPE_NAME_U64
            )));
        } else if type_name == TYPE_NAME_FLOAT {
            return Err(self.error(ErrorCode::UnknownType, &format!(
                "'{}' is not a valid type specifier. Use '{}' or '{}' instead",
                TYPE_NAME_FLOAT, TYPE_NAME_F32, TYPE_NAME_F64
            )));
        } else if type_name == TYPE_NAME_UNKNOWN {
            return Err(self.error_previous(ErrorCode::UnknownType, &format!(
                "'{}' is not a valid type specifier",
                TYPE_NAME_UNKNOWN
            )));
        }
        if let Some(symbol) = self.context.lookup_symbol(&type_name) {
            if symbol.kind == SymbolKind::Type {
                Ok(symbol.type_id.clone())
            } else {
                Err(self.error_previous(ErrorCode::UnknownType, &format!("'{}' is not a type name", type_name)))
            }
        } else {
            Err(self.error_previous(ErrorCode::UnknownType, &format!("Unknown type: {}", type_name)))
        }
    }

    /// Creates a SourceLocation from a token's position
    fn source_location_from_token(
        &self,
        token: &Token,
    ) -> slang_ir::source_location::SourceLocation {
        let (line, column) = self.line_info.get_line_col(token.pos);
        slang_ir::source_location::SourceLocation::new(token.pos, line, column, token.lexeme.len())
    }

    /// Consumes the current token if it matches the expected type
    ///
    /// ### Arguments
    ///
    /// * `token_type` - The token type to match
    ///
    /// ### Returns
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
    /// ### Arguments
    ///
    /// * `types` - The token types to match
    ///
    /// ### Returns
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
    /// ### Arguments
    ///
    /// * `token_type` - The token type to check for
    ///
    /// ### Returns
    ///
    /// true if the current token matches, false otherwise
    fn check(&self, token_type: &Tokentype) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    /// Checks if the next token matches the given type (lookahead of 2)
    ///
    /// ### Arguments
    ///
    /// * `token_type` - The token type to check against
    ///
    /// ### Returns
    ///
    /// true if the next token matches, false otherwise
    fn check_next(&self, token_type: &Tokentype) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        self.tokens[self.current + 1].token_type == *token_type
    }

    /// Advances to the next token and returns the previous token
    ///
    /// ### Returns
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
    /// ### Returns
    ///
    /// true if all tokens have been procesed, false otherwise
    #[inline]
    fn is_at_end(&self) -> bool {
        self.peek().token_type == Tokentype::Eof
    }

    /// Returns the current token without consuming it
    ///
    /// ### Returns
    ///
    /// The current token
    #[inline]
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Returns the most recently consumed token
    ///
    /// ### Returns
    ///
    /// The previous token
    #[inline]
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Parses an assignment statement
    ///
    /// ### Returns
    ///
    /// The parsed assignment statement or an error message
    fn assignment_statement(&mut self) -> Result<Statement, ParseError> {
        if !self.check(&Tokentype::Identifier) {
            return Err(self.error(ErrorCode::ExpectedIdentifier, "Expected identifier for assignment"));
        }

        let token_pos = self.peek().pos;
        let (line, column) = self.line_info.get_line_col(token_pos);

        let token = self.advance();
        let name = token.lexeme.clone();
        let location = slang_ir::source_location::SourceLocation::new(token_pos, line, column, name.len());

        if !self.match_token(&Tokentype::Equal) {
            return Err(self.error(ErrorCode::ExpectedEquals, "Expected '=' for assignment"));
        }

        let value = self.expression()?;

        if !self.match_token(&Tokentype::Semicolon) {
            return Err(self.error(ErrorCode::ExpectedSemicolon, "Expected ';' after assignment"));
        }

        Ok(Statement::Assignment(slang_ir::ast::AssignmentStatement {
            name,
            value,
            location,
        }))
    }

    /// Parses a conditional expression (if/else expression)
    ///
    /// ### Returns
    ///
    /// The parsed conditional expression or an error message
    fn conditional_expression(&mut self) -> Result<Expression, ParseError> {
        let if_token_pos = self.previous().pos;
        let (line, column) = self.line_info.get_line_col(if_token_pos);
        
        let condition = self.expression()?;
        
        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(self.error(ErrorCode::ExpectedOpeningBrace, "Expected '{' after if condition"));
        }
        
        let then_branch = self.expression()?;
        
        if !self.match_token(&Tokentype::RightBrace) {
            return Err(self.error(ErrorCode::ExpectedClosingBrace, "Expected '}' after if expression"));
        }
        
        if !self.match_token(&Tokentype::Else) {
            return Err(self.error(ErrorCode::ExpectedElse, "Expected 'else' after if expression"));
        }
        
        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(self.error(ErrorCode::ExpectedOpeningBrace, "Expected '{' after else"));
        }
        
        let else_branch = self.expression()?;
        
        if !self.match_token(&Tokentype::RightBrace) {
            return Err(self.error(ErrorCode::ExpectedClosingBrace, "Expected '}' after else expression"));
        }
        
        let end_pos = self.previous().pos + self.previous().lexeme.len();
        let location = slang_ir::source_location::SourceLocation::new(
            if_token_pos, 
            line, 
            column, 
            end_pos - if_token_pos
        );
        
        Ok(Expression::Conditional(ConditionalExpr {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
            expr_type: TypeId(PrimitiveType::Unknown as usize), 
            location,
        }))
    }

    /// Parses an if statement (if/else statement)
    ///
    /// ### Returns
    ///
    /// The parsed if statement or an error message
    fn if_statement(&mut self) -> Result<Statement, ParseError> {
        let if_token_pos = self.previous().pos;
        let (line, column) = self.line_info.get_line_col(if_token_pos);
        
        let condition = self.expression()?;
        
        if !self.match_token(&Tokentype::LeftBrace) {
            return Err(self.error(ErrorCode::ExpectedOpeningBrace, "Expected '{' after if condition"));
        }
        
        let mut then_branch = Vec::new();
        while !self.check(&Tokentype::RightBrace) && !self.is_at_end() {
            then_branch.push(self.statement()?);
        }
        
        if !self.match_token(&Tokentype::RightBrace) {
            return Err(self.error(ErrorCode::ExpectedClosingBrace, "Expected '}' after if body"));
        }
        
        let else_branch = if self.match_token(&Tokentype::Else) {
            if !self.match_token(&Tokentype::LeftBrace) {
                return Err(self.error(ErrorCode::ExpectedOpeningBrace, "Expected '{' after else"));
            }
            
            let mut else_statements = Vec::new();
            while !self.check(&Tokentype::RightBrace) && !self.is_at_end() {
                else_statements.push(self.statement()?);
            }
            
            if !self.match_token(&Tokentype::RightBrace) {
                return Err(self.error(ErrorCode::ExpectedClosingBrace, "Expected '}' after else body"));
            }
            
            Some(else_statements)
        } else {
            None
        };
        
        let end_pos = self.previous().pos + self.previous().lexeme.len();
        let location = slang_ir::source_location::SourceLocation::new(
            if_token_pos, 
            line, 
            column, 
            end_pos - if_token_pos
        );
        
        Ok(Statement::If(IfStatement {
            condition,
            then_branch,
            else_branch,
            location,
        }))
    }
}
