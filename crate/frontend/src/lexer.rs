use crate::token::{Token, Tokentype};
use slang_error::{CompileResult, CompilerError, ErrorCode, LineInfo};

pub struct LexerResult<'a> {
    /// The list of tokens generated from the input
    pub tokens: Vec<Token>,
    /// The line information for the tokens
    pub line_info: LineInfo<'a>,
}

/// Lexer for tracking position during tokenization
pub struct Lexer<'a> {
    /// Source text being tokenized
    input: &'a str,
    /// Iterator over source characters
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    /// Current position in source
    current_pos: usize,
    /// Current line number
    current_line: usize,
    /// Number of tokens on current line
    tokens_on_current_line: usize,
    /// Tokens generated so far
    tokens: Vec<Token>,
    /// Line token counts for line info
    line_tokens: Vec<(u16, u16)>,
    /// Collected lexer errors
    errors: Vec<CompilerError>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer state for the given input
    ///
    /// ### Arguments
    /// * `input` - The source code to tokenize
    ///
    /// ### Returns
    /// A new LexerState object
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.chars().peekable(),
            current_pos: 0,
            current_line: 1,
            tokens_on_current_line: 0,
            tokens: Vec::new(),
            line_tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Advances to the next character in the input
    ///
    /// ### Arguments
    /// * `state` - The current lexer state
    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c.is_some() {
            self.current_pos += 1;
        }
        c
    }

    /// Peeks at the next character without consuming it
    ///
    /// ### Arguments
    /// * `state` - The current lexer state
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Adds a token to the token list
    ///
    /// ### Arguments
    /// * `state` - The current lexer state
    /// * `token_type` - The type of token to add
    /// * `lexeme` - The string representation of the token
    /// * `start_pos` - The starting position of the token in the input
    fn add_token(&mut self, token_type: Tokentype, lexeme: String, start_pos: usize) {
        self.tokens.push(Token::new(token_type, lexeme, start_pos));
        self.tokens_on_current_line += 1;
    }

    /// Adds a token with suffix to the list
    ///
    /// ### Arguments
    /// * `state` - The current lexer state
    /// * `token_type` - The type of token to add
    /// * `lexeme` - The string representation of the token
    /// * `start_pos` - The starting position of the token in the input
    /// * `suffix` - The optional suffix for numeric literals
    fn add_token_with_suffix(&mut self, token_type: Tokentype, lexeme: String, start_pos: usize, suffix: Option<String>) {
        self.tokens.push(Token::new_with_suffix(token_type, lexeme, start_pos, suffix));
        self.tokens_on_current_line += 1;
    }

    /// Adds an error to the error list
    ///
    /// ### Arguments
    /// * `state` - The current lexer state
    /// * `error_code` - The error code for this error
    /// * `message` - The error message
    /// * `start_pos` - The starting position of the error
    /// * `token_length` - The length of the problematic token
    fn add_error(
        &mut self,
        error_code: ErrorCode,
        message: String,
        start_pos: usize,
        token_length: Option<usize>,
    ) {
        // Calculate column position from start_pos
        let line_start = self.input[..start_pos].rfind('\n').map_or(0, |pos| pos + 1);
        let column = start_pos - line_start + 1;

        self.errors.push(CompilerError::new(
            error_code,
            message,
            self.current_line,
            column,
            start_pos,
            token_length,
        ));
    }

    /// Records a line break, updating line counts
    ///
    /// ### Arguments
    /// * `state` - The current lexer state
    fn record_line_break(&mut self) {
        if self.tokens_on_current_line > 0 {
            self.line_tokens
                .push((self.current_line as u16, self.tokens_on_current_line as u16));
        }
        self.current_line += 1;
        self.tokens_on_current_line = 0;
    }

    /// Finishes tokenization and returns the result
    ///
    /// ### Arguments
    /// * `state` - The current lexer state
    fn finish(mut self) -> CompileResult<LexerResult<'a>> {
        // Add any remaining tokens on the last line
        if self.tokens_on_current_line > 0 {
            self.line_tokens
                .push((self.current_line as u16, self.tokens_on_current_line as u16));
        }
        self.tokens
            .push(Token::new(Tokentype::Eof, "".to_string(), self.current_pos));
        let mut info = LineInfo::new(self.input);
        info.per_line = self.line_tokens;

        // If there are errors, return them
        if !self.errors.is_empty() {
            return Err(self.errors);
        }

        Ok(LexerResult {
            tokens: self.tokens,
            line_info: info,
        })
    }

    pub fn tokenize(mut self) -> CompileResult<LexerResult<'a>> {
        while let Some(&c) = self.peek() {
            let token_start_pos = self.current_pos;

            match c {
                c if c.is_whitespace() => handle_whitespace(&mut self),
                c if c.is_alphabetic() => handle_identifier(&mut self, token_start_pos),
                c if c.is_ascii_digit() => handle_number(&mut self, token_start_pos),
                '"' => handle_string(&mut self),
                ':' => handle_simple_token(&mut self, Tokentype::Colon, ":", token_start_pos),
                '+' => handle_simple_token(&mut self, Tokentype::Plus, "+", token_start_pos),
                '-' => handle_dash(&mut self, token_start_pos),
                '*' => handle_simple_token(&mut self, Tokentype::Multiply, "*", token_start_pos),
                '/' => handle_slash(&mut self, token_start_pos),
                '=' => handle_equals(&mut self, token_start_pos),
                '<' => handle_less_than(&mut self, token_start_pos),
                '>' => handle_greater_than(&mut self, token_start_pos),
                '!' => handle_exclamation(&mut self, token_start_pos),
                ';' => handle_simple_token(&mut self, Tokentype::Semicolon, ";", token_start_pos),
                '{' => handle_simple_token(&mut self, Tokentype::LeftBrace, "{", token_start_pos),
                '}' => handle_simple_token(&mut self, Tokentype::RightBrace, "}", token_start_pos),
                ',' => handle_simple_token(&mut self, Tokentype::Comma, ",", token_start_pos),
                '(' => handle_simple_token(&mut self, Tokentype::LeftParen, "(", token_start_pos),
                ')' => handle_simple_token(&mut self, Tokentype::RightParen, ")", token_start_pos),
                '&' => handle_ampersand(&mut self, token_start_pos),
                '|' => handle_pipe(&mut self, token_start_pos),
                _ => handle_invalid_char(&mut self, token_start_pos),
            }
        }

        self.finish()
    }
}

/// Handles whitespace characters in the input
///
/// ### Arguments
/// * `state` - The current lexer state
fn handle_whitespace(state: &mut Lexer) {
    let c = state.advance().unwrap();

    if c == '\n' {
        state.record_line_break();
    }
}

/// Handles alphabetic identifiers and keywords
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the identifier in the input
fn handle_identifier(state: &mut Lexer, start_pos: usize) {
    let mut identifier = String::new();

    while let Some(&c) = state.peek() {
        if c.is_alphanumeric() || c == '_' {
            identifier.push(c);
            state.advance();
        } else {
            break;
        }
    }

    let token_type = match identifier.as_str() {
        "let" => Tokentype::Let,
        "mut" => Tokentype::Mut,
        "struct" => Tokentype::Struct,
        "fn" => Tokentype::Fn,
        "return" => Tokentype::Return,
        "if" => Tokentype::If,
        "else" => Tokentype::Else,
        "true" | "false" => Tokentype::BooleanLiteral,
        _ => Tokentype::Identifier,
    };

    state.add_token(token_type, identifier, start_pos);
}

/// Handles numeric literals (integers and floating point) with optional type suffix
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the number in the input
fn handle_number(state: &mut Lexer, start_pos: usize) {
    let mut number = String::new();
    let mut is_float = false;

    // Parse the numeric part
    while let Some(&c) = state.peek() {
        if c.is_ascii_digit() {
            number.push(c);
            state.advance();
        } else if c == '.' {
            if is_float {
                break;
            }
            is_float = true;
            number.push(c);
            state.advance();
        } else if c == 'e' || c == 'E' {
            number.push(c);
            state.advance();
            if let Some(&next_c) = state.peek() {
                if next_c == '+' || next_c == '-' {
                    number.push(next_c);
                    state.advance();
                }
            }
        } else {
            break;
        }
    }

    // Check for type suffix (i32, i64, u32, u64, f32, f64)
    let mut suffix = None;
    if let Some(&c) = state.peek() {
        if c.is_ascii_alphabetic() {
            let mut potential_suffix = String::new();
            
            // Collect potential suffix characters
            while let Some(&c) = state.peek() {
                if c.is_ascii_alphanumeric() {
                    potential_suffix.push(c);
                    state.advance();
                } else {
                    break;
                }
            }
            
            // Validate the suffix - only allow known type suffixes
            match potential_suffix.as_str() {
                "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => {
                    suffix = Some(potential_suffix);
                }
                _ => {
                    // This is not a valid type suffix, so we need to rewind
                    // the lexer position back to before the suffix
                    for _ in 0..potential_suffix.len() {
                        if state.current_pos > 0 {
                            state.current_pos -= 1;
                        }
                    }
                    // Reset the peekable iterator by recreating it from the current position
                    let remaining_input = &state.input[state.current_pos..];
                    state.chars = remaining_input.chars().peekable();
                }
            }
        }
    }

    let token_type = if is_float {
        Tokentype::FloatLiteral
    } else {
        Tokentype::IntegerLiteral
    };

    state.add_token_with_suffix(token_type, number, start_pos, suffix);
}

/// Handles string literals
///
/// ### Arguments
/// * `state` - The current lexer state
fn handle_string(state: &mut Lexer) {
    let start_pos = state.current_pos;
    state.advance(); // consume opening quote
    let mut string = String::new();
    let mut closed = false;

    while let Some(&c) = state.peek() {
        if c == '"' {
            state.advance();
            closed = true;
            break;
        } else if c == '\n' {
            state.current_line += 1;
            string.push(c);
            state.advance();
        } else {
            string.push(c);
            state.advance();
        }
    }

    if !closed {
        let error_message = "Expected closing quote for string literal".to_string();
        let invalid_lexeme = format!("\"{string}");
        state.add_error(
            ErrorCode::ExpectedClosingQuote,
            error_message,
            start_pos,
            Some(invalid_lexeme.len()),
        );
    } else {
        state.add_token(Tokentype::StringLiteral, string, start_pos);
    }
}

/// Handles simple one-character tokens
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `token_type` - The type of token to add
/// * `lexeme` - The string representation of the token
/// * `start_pos` - The starting position of the token in the input
fn handle_simple_token(
    state: &mut Lexer,
    token_type: Tokentype,
    lexeme: &str,
    start_pos: usize,
) {
    state.advance();
    state.add_token(token_type, lexeme.to_string(), start_pos);
}

/// Handles dash character (minus or arrow)
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the dash in the input
fn handle_dash(state: &mut Lexer, start_pos: usize) {
    state.advance();
    if state.peek() == Some(&'>') {
        state.advance();
        state.add_token(Tokentype::Arrow, "->".to_string(), start_pos);
    } else {
        state.add_token(Tokentype::Minus, "-".to_string(), start_pos);
    }
}

/// Handles slash character (divide or comments)
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the slash in the input
fn handle_slash(state: &mut Lexer, start_pos: usize) {
    state.advance();

    if state.peek() == Some(&'/') {
        handle_line_comment(state);
    } else if state.peek() == Some(&'*') {
        handle_block_comment(state);
    } else {
        state.add_token(Tokentype::Divide, "/".to_string(), start_pos);
    }
}

/// Handles single-line comments
///
/// ### Arguments
/// * `state` - The current lexer state
fn handle_line_comment(state: &mut Lexer) {
    state.advance();

    while let Some(&c) = state.peek() {
        if c == '\n' {
            state.advance();
            state.record_line_break();
            break;
        }
        state.advance();
    }
}

/// Handles multi-line block comments
///
/// ### Arguments
/// * `state` - The current lexer state
fn handle_block_comment(state: &mut Lexer) {
    state.advance();

    let mut nesting = 1;
    while nesting > 0 {
        if state.peek().is_none() {
            break;
        }

        if let Some(&c) = state.peek() {
            if c == '\n' {
                state.record_line_break();
            }
        }

        if state.peek() == Some(&'*') {
            state.advance();
            if state.peek() == Some(&'/') {
                state.advance();
                nesting -= 1;
                continue;
            }
        } else if state.peek() == Some(&'/') {
            state.advance();
            if state.peek() == Some(&'*') {
                state.advance();
                nesting += 1;
                continue;
            }
        } else {
            state.advance();
        }
    }
}

/// Handles equals character (assignment or equality)
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the equals in the input
fn handle_equals(state: &mut Lexer, start_pos: usize) {
    state.advance();
    if state.peek() == Some(&'=') {
        state.advance();
        state.add_token(Tokentype::EqualEqual, "==".to_string(), start_pos);
    } else {
        state.add_token(Tokentype::Equal, "=".to_string(), start_pos);
    }
}

/// Handles less than character (less than or less than or equal)
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the less than in the input
fn handle_less_than(state: &mut Lexer, start_pos: usize) {
    state.advance();
    if state.peek() == Some(&'=') {
        state.advance();
        state.add_token(Tokentype::LessEqual, "<=".to_string(), start_pos);
    } else {
        state.add_token(Tokentype::Less, "<".to_string(), start_pos);
    }
}

/// Handles greater than character (greater than or greater than or equal)
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the greater than in the input
fn handle_greater_than(state: &mut Lexer, start_pos: usize) {
    state.advance();
    if state.peek() == Some(&'=') {
        state.advance();
        state.add_token(Tokentype::GreaterEqual, ">=".to_string(), start_pos);
    } else {
        state.add_token(Tokentype::Greater, ">".to_string(), start_pos);
    }
}

/// Handles exclamation mark (not or not equal)
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the exclamation mark in the input
fn handle_exclamation(state: &mut Lexer, start_pos: usize) {
    state.advance();
    if state.peek() == Some(&'=') {
        state.advance();
        state.add_token(Tokentype::NotEqual, "!=".to_string(), start_pos);
    } else {
        state.add_token(Tokentype::Not, "!".to_string(), start_pos);
    }
}

/// Handles ampersand character (logical AND)
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the ampersand in the input
fn handle_ampersand(state: &mut Lexer, start_pos: usize) {
    state.advance();
    if state.peek() == Some(&'&') {
        state.advance();
        state.add_token(Tokentype::And, "&&".to_string(), start_pos);
    } else {
        state.add_token(Tokentype::Invalid, "&".to_string(), start_pos);
    }
}

/// Handles pipe character (logical OR)
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the pipe in the input
fn handle_pipe(state: &mut Lexer, start_pos: usize) {
    state.advance();
    if state.peek() == Some(&'|') {
        state.advance();
        state.add_token(Tokentype::Or, "||".to_string(), start_pos);
    } else {
        state.add_token(Tokentype::Invalid, "|".to_string(), start_pos);
    }
}

/// Handles invalid characters
///
/// ### Arguments
/// * `state` - The current lexer state
/// * `start_pos` - The starting position of the invalid character in the input
fn handle_invalid_char(state: &mut Lexer, start_pos: usize) {
    let invalid_char = state.advance().unwrap();
    state.add_token(Tokentype::Invalid, invalid_char.to_string(), start_pos);
}
