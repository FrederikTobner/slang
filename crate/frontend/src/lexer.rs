use crate::error::LineInfo;
use crate::token::{Token, Tokentype};

pub struct Result<'a> {
    /// The list of tokens generated from the input
    pub tokens: Vec<Token>,
    /// The line information for the tokens
    pub line_info: LineInfo<'a>,
}

/// Lexer state for tracking position during tokenization
struct LexerState<'a> {
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
}

impl<'a> LexerState<'a> {
    /// Creates a new lexer state for the given input
    ///
    /// ### Arguments
    /// * `input` - The source code to tokenize
    ///
    /// ### Returns
    /// A new LexerState object
    fn new(input: &'a str) -> Self {
        LexerState {
            input,
            chars: input.chars().peekable(),
            current_pos: 0,
            current_line: 1,
            tokens_on_current_line: 0,
            tokens: Vec::new(),
            line_tokens: Vec::new(),
        }
    }

    /// Advances to the next character in the input
    ///
    /// ### Arguments
    /// * `self` - The current lexer state
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
    /// * `self` - The current lexer state
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Adds a token to the token list
    ///
    /// ### Arguments
    /// * `self` - The current lexer state
    /// * `token_type` - The type of token to add
    /// * `lexeme` - The string representation of the token
    /// * `start_pos` - The starting position of the token in the input
    fn add_token(&mut self, token_type: Tokentype, lexeme: String, start_pos: usize) {
        self.tokens.push(Token::new(token_type, lexeme, start_pos));
        self.tokens_on_current_line += 1;
    }

    /// Records a line break, updating line counts
    ///
    /// ### Arguments
    /// * `self` - The current lexer state
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
    /// * `self` - The current lexer state
    fn finish(mut self) -> Result<'a> {
        // Add any remaining tokens on the last line
        if self.tokens_on_current_line > 0 {
            self.line_tokens
                .push((self.current_line as u16, self.tokens_on_current_line as u16));
        }
        self.tokens
            .push(Token::new(Tokentype::Eof, "".to_string(), self.current_pos));
        let mut info = LineInfo::new(self.input);
        info.per_line = self.line_tokens;

        Result {
            tokens: self.tokens,
            line_info: info,
        }
    }
}

/// Converts source code text into a sequence of tokens with line information
///
/// ### Arguments
///
/// * `input` - The source code to tokenize
///
/// ### Returns
///
/// A LexerResult containing tokens and line information
pub fn tokenize(input: &str) -> Result {
    let mut state = LexerState::new(input);

    while let Some(&c) = state.peek() {
        let token_start_pos = state.current_pos;

        match c {
            c if c.is_whitespace() => handle_whitespace(&mut state),
            c if c.is_alphabetic() => handle_identifier(&mut state, token_start_pos),
            c if c.is_ascii_digit() => handle_number(&mut state, token_start_pos),
            '"' => handle_string(&mut state),
            ':' => handle_simple_token(&mut state, Tokentype::Colon, ":", token_start_pos),
            '+' => handle_simple_token(&mut state, Tokentype::Plus, "+", token_start_pos),
            '-' => handle_dash(&mut state, token_start_pos),
            '*' => handle_simple_token(&mut state, Tokentype::Multiply, "*", token_start_pos),
            '/' => handle_slash(&mut state, token_start_pos),
            '=' => handle_equals(&mut state, token_start_pos),
            '<' => handle_less_than(&mut state, token_start_pos),
            '>' => handle_greater_than(&mut state, token_start_pos),
            '!' => handle_exclamation(&mut state, token_start_pos),
            ';' => handle_simple_token(&mut state, Tokentype::Semicolon, ";", token_start_pos),
            '{' => handle_simple_token(&mut state, Tokentype::LeftBrace, "{", token_start_pos),
            '}' => handle_simple_token(&mut state, Tokentype::RightBrace, "}", token_start_pos),
            ',' => handle_simple_token(&mut state, Tokentype::Comma, ",", token_start_pos),
            '(' => handle_simple_token(&mut state, Tokentype::LeftParen, "(", token_start_pos),
            ')' => handle_simple_token(&mut state, Tokentype::RightParen, ")", token_start_pos),
            '&' => handle_ampersand(&mut state, token_start_pos),
            '|' => handle_pipe(&mut state, token_start_pos),
            _ => handle_invalid_char(&mut state, token_start_pos),
        }
    }

    state.finish()
}

/// Handles whitespace characters in the input
///
/// ### Arguments
/// * `self` - The current lexer state
fn handle_whitespace(state: &mut LexerState) {
    let c = state.advance().unwrap();

    if c == '\n' {
        state.record_line_break();
    }
}

/// Handles alphabetic identifiers and keywords
///
/// ### Arguments
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the identifier in the input
fn handle_identifier(state: &mut LexerState, start_pos: usize) {
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
        "true" | "false" => Tokentype::BooleanLiteral,
        _ => Tokentype::Identifier,
    };

    state.add_token(token_type, identifier, start_pos);
}

/// Handles numeric literals (integers and floating point)
///
/// ### Arguments
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the number in the input
fn handle_number(state: &mut LexerState, start_pos: usize) {
    let mut number = String::new();
    let mut is_float = false;

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

    let token_type = if is_float {
        Tokentype::FloatLiteral
    } else {
        Tokentype::IntegerLiteral
    };

    state.add_token(token_type, number, start_pos);
}

/// Handles string literals
///
/// ### Arguments
/// * `self` - The current lexer state
fn handle_string(state: &mut LexerState) {
    state.advance(); // Skip opening quote
    let mut string = String::new();
    let start_pos = state.current_pos;

    while let Some(&c) = state.peek() {
        if c == '"' {
            state.advance();
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

    state.add_token(Tokentype::StringLiteral, string, start_pos);
}

/// Handles simple one-character tokens
///
/// ### Arguments
/// * `self` - The current lexer state
/// * `token_type` - The type of token to add
/// * `lexeme` - The string representation of the token
/// * `start_pos` - The starting position of the token in the input
fn handle_simple_token(
    state: &mut LexerState,
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
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the dash in the input
fn handle_dash(state: &mut LexerState, start_pos: usize) {
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
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the slash in the input
fn handle_slash(state: &mut LexerState, start_pos: usize) {
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
/// * `self` - The current lexer state
fn handle_line_comment(state: &mut LexerState) {
    state.advance(); // Skip the second '/'

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
/// * `self` - The current lexer state
fn handle_block_comment(state: &mut LexerState) {
    state.advance(); // Skip the '*'

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
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the equals in the input
fn handle_equals(state: &mut LexerState, start_pos: usize) {
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
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the less than in the input
fn handle_less_than(state: &mut LexerState, start_pos: usize) {
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
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the greater than in the input
fn handle_greater_than(state: &mut LexerState, start_pos: usize) {
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
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the exclamation mark in the input
fn handle_exclamation(state: &mut LexerState, start_pos: usize) {
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
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the ampersand in the input
fn handle_ampersand(state: &mut LexerState, start_pos: usize) {
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
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the pipe in the input
fn handle_pipe(state: &mut LexerState, start_pos: usize) {
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
/// * `self` - The current lexer state
/// * `start_pos` - The starting position of the invalid character in the input
fn handle_invalid_char(state: &mut LexerState, start_pos: usize) {
    let invalid_char = state.advance().unwrap();
    state.add_token(Tokentype::Invalid, invalid_char.to_string(), start_pos);
}
