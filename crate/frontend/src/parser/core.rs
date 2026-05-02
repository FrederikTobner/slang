// Core parser infrastructure module
// Contains the main Parser struct and fundamental parsing methods

use super::expressions::ExpressionParsing;
use super::literals::LiteralParsing;
use super::statements::StatementParsing;
use super::types::TypeParsing;
use crate::token::{Token, Tokentype};
use slang_error::ParseError;
use slang_error::{CompilationError, CompileResult, DomainError, LineInfo};
use slang_ir::ast::{BinaryOperator, BlockExpr, Expression, Statement};
use slang_shared::CompilationContext;
use slang_types::TypeId;

/// Position information extracted from tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenPosition {
    /// Starting position in source
    pub pos: usize,
    /// Length of the token
    pub len: usize,
}

impl TokenPosition {
    /// Create a new TokenPosition
    pub fn new(pos: usize, len: usize) -> Self {
        TokenPosition { pos, len }
    }

    /// Convert to a source location using line info
    pub fn to_location(self, line_info: &LineInfo) -> slang_error::location::Location {
        let (line, column) = line_info.get_line_col(self.pos);
        slang_error::location::Location::new(self.pos, line, column, self.len)
    }

    /// Calculate end position
    pub fn end_pos(self) -> usize {
        self.pos + self.len
    }
}

/// Parser that converts tokens into an abstract syntax tree
pub struct Parser<'a> {
    /// The tokens being parsed
    pub(super) tokens: &'a [Token],
    /// Current position in the token list
    pub(super) current: usize,
    /// Line information for error reporting
    pub(super) line_info: &'a LineInfo<'a>,
    /// Errors collected during parsing
    pub(super) errors: Vec<CompilationError>,
    /// Compilation context for type information
    pub(super) context: &'a mut CompilationContext,
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given tokens and line information
    ///
    /// ### Arguments
    ///
    /// * `tokens` - The tokens to parse
    /// * `line_info` - Line information for error reporting
    /// * `context` - The compilation context
    pub fn new(
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
    pub fn parse(&mut self) -> CompileResult<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.errors.push(e.to_compiler_error());
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

    // Token management methods

    /// Advances to the next token and returns the previous token
    ///
    /// ### Returns
    ///
    /// The token that was current before advancing, if the end of the token stream was not reached
    /// Otherwise, returns the last token
    pub(super) fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Returns the current token without consuming it
    ///
    /// ### Returns
    ///
    /// The current token
    #[inline]
    pub(super) fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Returns the most recently consumed token
    ///
    /// ### Returns
    ///
    /// The previous token
    #[inline]
    pub(super) fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Checks if we've reached the end of the token stream
    ///
    /// ### Returns
    ///
    /// true if all tokens have been procesed, false otherwise
    #[inline]
    pub(super) fn is_at_end(&self) -> bool {
        self.peek().token_type == Tokentype::Eof
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
    pub(super) fn check(&self, token_type: &Tokentype) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
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
    pub(super) fn match_token(&mut self, token_type: &Tokentype) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
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
    pub(super) fn check_next(&self, token_type: &Tokentype) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        self.tokens[self.current + 1].token_type == *token_type
    }

    // Error handling methods - removed helper methods, use ParseErrorFactory directly

    /// Helper to get current token location
    pub(super) fn current_location(&self) -> slang_error::Location {
        let current_token = self.peek();
        TokenPosition::new(current_token.pos, current_token.lexeme.len())
            .to_location(self.line_info)
    }

    /// Skip tokens until a safe synchronization point for error recovery
    ///
    /// This function helps the parser recover from errors by advancing to the next
    /// statement boundary, allowing parsing to continue after an error.
    pub(super) fn synchronize(&mut self) {
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

    /// Creates a SourceLocation from a token's position and line information
    ///
    /// ### Arguments
    ///
    /// * `token` - The token to create location information for
    ///
    /// ### Returns
    ///
    /// A Location struct with line, column, position and length information
    pub(super) fn source_location_from_token(
        &self,
        token: &Token,
    ) -> slang_error::location::Location {
        let (line, column) = self.line_info.get_line_col(token.pos);
        slang_error::location::Location::new(token.pos, line, column, token.lexeme.len())
    }

    /// Creates a Location spanning from one position to another
    ///
    /// ### Arguments
    ///
    /// * `start_pos` - The starting position
    /// * `end_pos` - The ending position (exclusive)
    ///
    /// ### Returns
    ///
    /// A Location struct covering the range from start_pos to end_pos
    pub(super) fn location_from_range(
        &self,
        start_pos: usize,
        end_pos: usize,
    ) -> slang_error::location::Location {
        let (start_line, start_column) = self.line_info.get_line_col(start_pos);
        let length = end_pos - start_pos;
        slang_error::location::Location::new(start_pos, start_line, start_column, length)
    }

    /// Parses a single statement
    ///
    /// ### Returns
    ///
    /// The parsed statement or an error message
    pub(super) fn statement(&mut self) -> Result<Statement, ParseError> {
        // Will be implemented when StatementParsing trait is complete
        StatementParsing::statement(self)
    }

    pub(super) fn expression(&mut self) -> Result<Expression, ParseError> {
        ExpressionParsing::expression(self)
    }

    pub(super) fn parse_type(&mut self) -> Result<TypeId, ParseError> {
        TypeParsing::parse_type(self)
    }

    pub(super) fn parse_integer(&mut self) -> Result<Expression, ParseError> {
        LiteralParsing::parse_integer(self)
    }

    pub(super) fn parse_float(&mut self) -> Result<Expression, ParseError> {
        LiteralParsing::parse_float(self)
    }

    pub(super) fn finish_call(
        &mut self,
        name: String,
        name_location: slang_error::location::Location,
    ) -> Result<Expression, ParseError> {
        ExpressionParsing::finish_call(self, name, name_location)
    }

    pub(super) fn conditional_expression(&mut self) -> Result<Expression, ParseError> {
        ExpressionParsing::conditional_expression(self)
    }

    pub(super) fn parse_block_expression(&mut self) -> Result<BlockExpr, ParseError> {
        ExpressionParsing::parse_block_expression(self)
    }

    pub(super) fn parse_function_type_expression(&mut self) -> Result<Expression, ParseError> {
        TypeParsing::parse_function_type_expression(self)
    }

    /// Enhanced token matching that returns a reference to the matched token
    ///
    /// ### Arguments
    /// * `token_type` - The token type to match against
    ///
    /// ### Returns
    /// * `Some(&Token)` - Reference to the consumed token if match succeeded
    /// * `None` - If the current token doesn't match the expected type
    #[inline]
    pub(super) fn match_token_ref(&mut self, token_type: &Tokentype) -> Option<&Token> {
        if self.check(token_type) {
            Some(self.advance()) // advance() already returns &Token
        } else {
            None
        }
    }

    /// Enhanced multi-token matching with captured token reference
    ///
    /// ### Arguments
    /// * `types` - Array of token types to match against
    ///
    /// ### Returns
    /// * `Some(&Token)` - Reference to the matched token
    /// * `None` - If no token types matched
    #[inline]
    pub(super) fn match_any_ref(&mut self, types: &[Tokentype]) -> Option<&Token> {
        for token_type in types.iter() {
            if self.check(token_type) {
                return Some(self.advance());
            }
        }
        None
    }

    /// Match integer literal token and return the token for use with parse_integer
    pub(super) fn match_integer_literal_token(&mut self) -> bool {
        self.match_token(&Tokentype::IntegerLiteral)
    }

    /// Match identifier token and return its name with position
    pub(super) fn match_identifier_token(&mut self) -> Option<(&str, TokenPosition)> {
        if let Some(token) = self.match_token_ref(&Tokentype::Identifier) {
            Some((
                &token.lexeme,
                TokenPosition {
                    pos: token.pos,
                    len: token.lexeme.len(),
                },
            ))
        } else {
            None
        }
    }

    /// Match string literal token and return its value with position
    pub(super) fn match_string_literal_token(&mut self) -> Option<(&str, TokenPosition)> {
        if let Some(token) = self.match_token_ref(&Tokentype::StringLiteral) {
            Some((
                &token.lexeme,
                TokenPosition {
                    pos: token.pos,
                    len: token.lexeme.len(),
                },
            ))
        } else {
            None
        }
    }

    /// Match boolean literal token and return its value with position
    pub(super) fn match_boolean_literal_token(&mut self) -> Option<(&str, TokenPosition)> {
        if let Some(token) = self.match_token_ref(&Tokentype::BooleanLiteral) {
            Some((
                &token.lexeme,
                TokenPosition {
                    pos: token.pos,
                    len: token.lexeme.len(),
                },
            ))
        } else {
            None
        }
    }

    /// Match equality operators (==, !=) specifically
    pub(super) fn match_equality_operator(&mut self) -> Option<(BinaryOperator, TokenPosition)> {
        if let Some(token) = self.match_any_ref(&[Tokentype::EqualEqual, Tokentype::NotEqual]) {
            let operator = match token.token_type {
                Tokentype::EqualEqual => BinaryOperator::Equal,
                Tokentype::NotEqual => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };
            Some((
                operator,
                TokenPosition {
                    pos: token.pos,
                    len: token.lexeme.len(),
                },
            ))
        } else {
            None
        }
    }

    /// Match comparison operators (>, <, >=, <=) specifically
    pub(super) fn match_comparison_operator(&mut self) -> Option<(BinaryOperator, TokenPosition)> {
        if let Some(token) = self.match_any_ref(&[
            Tokentype::Greater,
            Tokentype::GreaterEqual,
            Tokentype::Less,
            Tokentype::LessEqual,
        ]) {
            let operator = match token.token_type {
                Tokentype::Greater => BinaryOperator::GreaterThan,
                Tokentype::GreaterEqual => BinaryOperator::GreaterThanOrEqual,
                Tokentype::Less => BinaryOperator::LessThan,
                Tokentype::LessEqual => BinaryOperator::LessThanOrEqual,
                _ => unreachable!(),
            };
            Some((
                operator,
                TokenPosition {
                    pos: token.pos,
                    len: token.lexeme.len(),
                },
            ))
        } else {
            None
        }
    }

    /// Match term operators (+, -) specifically
    pub(super) fn match_term_operator(&mut self) -> Option<(BinaryOperator, TokenPosition)> {
        if let Some(token) = self.match_any_ref(&[Tokentype::Plus, Tokentype::Minus]) {
            let operator = match token.token_type {
                Tokentype::Plus => BinaryOperator::Add,
                Tokentype::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            Some((
                operator,
                TokenPosition {
                    pos: token.pos,
                    len: token.lexeme.len(),
                },
            ))
        } else {
            None
        }
    }

    /// Match factor operators (*, /) specifically
    pub(super) fn match_factor_operator(&mut self) -> Option<(BinaryOperator, TokenPosition)> {
        if let Some(token) = self.match_any_ref(&[Tokentype::Multiply, Tokentype::Divide]) {
            let operator = match token.token_type {
                Tokentype::Multiply => BinaryOperator::Multiply,
                Tokentype::Divide => BinaryOperator::Divide,
                _ => unreachable!(),
            };
            Some((
                operator,
                TokenPosition {
                    pos: token.pos,
                    len: token.lexeme.len(),
                },
            ))
        } else {
            None
        }
    }
}
