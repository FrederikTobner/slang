// Core parser infrastructure module
// Contains the main Parser struct and fundamental parsing methods

use super::error::ParseError;
use super::expressions::ExpressionParser;
use super::literals::LiteralParser;
use super::statements::StatementParser;
use super::types::TypeParser;
use super::utilities::UtilitiesParser;
use crate::token::{Token, Tokentype};
use slang_error::{CompileResult, CompilerError, ErrorCode, LineInfo};
use slang_ir::ast::{BlockExpr, Expression, Statement};
use slang_shared::CompilationContext;
use slang_types::TypeId;

/// Parser that converts tokens into an abstract syntax tree
pub struct Parser<'a> {
    /// The tokens being parsed
    pub(super) tokens: &'a [Token],
    /// Current position in the token list
    pub(super) current: usize,
    /// Line information for error reporting
    pub(super) line_info: &'a LineInfo<'a>,
    /// Errors collected during parsing
    pub(super) errors: Vec<CompilerError>,
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

    /// Consumes the current token if it matches any of the expected types
    ///
    /// ### Arguments
    ///
    /// * `types` - The token types to match
    ///
    /// ### Returns
    ///
    /// true if a token was consumed, false otherwise
    pub(super) fn match_any(&mut self, types: &[Tokentype]) -> bool {
        for token_type in types.iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
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

    // Error handling methods

    /// Creates an error at the current token position
    ///
    /// ### Arguments
    ///
    /// * `error_code` - The error code for the error
    /// * `message` - The error message to display
    ///
    /// ### Returns
    /// A new ParseError with the current token position and length
    pub(super) fn error(&self, error_code: ErrorCode, message: &str) -> ParseError {
        ParseError::new(
            error_code,
            message,
            self.peek().pos,
            self.peek().lexeme.len(),
        )
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
    pub(super) fn error_previous(&self, error_code: ErrorCode, message: &str) -> ParseError {
        ParseError::new(
            error_code,
            message,
            self.previous().pos,
            self.previous().lexeme.len(),
        )
    }

    /// Skip until a safe synchronization point (e.g., semicolon or statement start)
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

    // Utility methods

    /// Creates a SourceLocation from a token's position
    pub(super) fn source_location_from_token(&self, token: &Token) -> slang_ir::location::Location {
        let (line, column) = self.line_info.get_line_col(token.pos);
        slang_ir::location::Location::new(token.pos, line, column, token.lexeme.len())
    }

    // TEMPORARY: Include all original parsing methods for Phase 1 compatibility
    // These will be moved to their respective modules in later phases

    // Include all original methods from parser_old.rs here temporarily
    // This is a temporary measure to ensure the parser works in Phase 1

    /// Parses a single statement
    ///
    /// ### Returns
    ///
    /// The parsed statement or an error message
    pub(super) fn statement(&mut self) -> Result<Statement, ParseError> {
        StatementParser::parse_statement(self)
    }

    // Expression parsing methods now delegated to ExpressionParser

    pub(super) fn expression(&mut self) -> Result<Expression, ParseError> {
        ExpressionParser::parse_expression(self)
    }

    // Type and literal parsing methods (temporary - will move to respective modules in Phase 4)

    pub(super) fn parse_type(&mut self) -> Result<TypeId, ParseError> {
        TypeParser::parse_type(self)
    }

    pub(super) fn parse_integer(&mut self) -> Result<Expression, ParseError> {
        LiteralParser::parse_integer(self)
    }

    pub(super) fn parse_float(&mut self) -> Result<Expression, ParseError> {
        LiteralParser::parse_float(self)
    }

    pub(super) fn finish_call(&mut self, name: String) -> Result<Expression, ParseError> {
        UtilitiesParser::finish_call(self, name)
    }

    pub(super) fn conditional_expression(&mut self) -> Result<Expression, ParseError> {
        UtilitiesParser::conditional_expression(self)
    }

    pub(super) fn parse_block_expression(&mut self) -> Result<BlockExpr, ParseError> {
        UtilitiesParser::parse_block_expression(self)
    }

    pub(super) fn parse_function_type_expression(&mut self) -> Result<Expression, ParseError> {
        TypeParser::parse_function_type_expression(self)
    }
}
