use crate::token::{Token, Tokentype};

pub struct LexerResult<'a> {
    /// The list of tokens generated from the input
    pub tokens: Vec<Token>,
    /// The line information for the tokens
    pub line_info: LineInfo<'a>,
}

pub struct LineInfo<'a> {
    /// Number of tokens on each line (run-length encoded)
    /// (line_number, tokens_on_line)
    pub per_line: Vec<(u16, u16)>, 
    /// Reference to the original source code
    source: &'a str,
    /// The starting position of each line in the source code
    pub line_starts: Vec<usize>,
}

impl LineInfo<'_> {
    /// Creates a new LineInfo object
    /// 
    /// ### Arguments
    /// * `source` - The source code string
    /// 
    /// ### Returns
    /// A new LineInfo object with the line starts calculated
    pub fn new(source: &str) -> LineInfo {
        let mut line_starts = vec![0];

        for (i, c) in source.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }
        
        LineInfo {
            per_line: Vec::new(),
            source,
            line_starts,
        }
    }
    
    /// Get the line and column number for a token position
    /// 
    /// # Arguments
    /// * `pos` - The position of the token in the source code
    /// 
    /// # Returns
    /// A tuple containing the line number and column number
    pub fn get_line_col(&self, pos: usize) -> (usize, usize) {
        match self.line_starts.binary_search(&pos) {
            Ok(line) => (line + 1, 1),
            Err(line) => {
                let line_idx = line - 1;
                let col = pos - self.line_starts[line_idx] + 1;
                (line_idx + 1, col)
            }
        }
    }
    
    /// Get the text for a specific line
    /// 
    /// ### Arguments
    /// * `line` - The line number to retrieve
    /// 
    /// ### Returns
    /// The text of the line, or None if the line number is invalid
    pub fn get_line_text(&self, line: usize) -> Option<&str> {
        if line == 0 || line > self.line_starts.len() {
            return None;
        }
        
        let start = self.line_starts[line - 1];
        let end = if line < self.line_starts.len() {
            self.line_starts[line]
        } else {
            self.source.len()
        };
        
        let actual_end = if start < end && end > 0 && 
                           self.source.as_bytes().get(end - 1) == Some(&b'\n') {
            end - 1
        } else {
            end
        };
        
        Some(&self.source[start..actual_end])
    }
    
    /// Format an error message with line information and source code snippet
    pub fn format_error(&self, pos: usize, message: &str) -> String {
        let (line, col) = self.get_line_col(pos);
        let line_str = self.get_line_text(line).unwrap_or("");
        
        format!(
            "Error at line {}, column {}: {}\n{}\n{}^",
            line, col, message, line_str,
            " ".repeat(col - 1)
        )
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
pub fn tokenize(input: &str) -> LexerResult {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current_pos = 0;
    let mut current_line = 1;
    let mut tokens_on_current_line = 0;
    let mut line_tokens = Vec::new();
    
    while let Some(&c) = chars.peek() {
        let token_start_pos = current_pos;
        
        match c {
            c if c.is_whitespace() => {
                if c == '\n' {
                    if tokens_on_current_line > 0 {
                        line_tokens.push((current_line as u16, tokens_on_current_line as u16));
                    }
                    current_line += 1;
                    tokens_on_current_line = 0;
                }
                chars.next();
                current_pos += 1;
                continue;
            }

            c if c.is_alphabetic() => {
                let mut identifier = String::new();
                let start_pos = current_pos;

                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                        chars.next();
                        current_pos += 1;
                    } else {
                        break;
                    }
                }

                let token_type = match identifier.as_str() {
                    "let" => Tokentype::Let,
                    "struct" => Tokentype::Struct,
                    "fn" => Tokentype::Fn,
                    "return" => Tokentype::Return,
                    "true" | "false" => Tokentype::BooleanLiteral,
                    _ => Tokentype::Identifier,
                };
                
                tokens.push(Token::new(token_type, identifier, start_pos));
                tokens_on_current_line += 1;
            }

            c if c.is_ascii_digit() => {
                let mut number = String::new();
                let mut is_float = false;
                let start_pos = current_pos;
                
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        number.push(c);
                        chars.next();
                        current_pos += 1;
                    } else if c == '.' {
                        if is_float {
                            break; 
                        }
                        is_float = true;
                        number.push(c);
                        chars.next();
                        current_pos += 1;
                    } else if c == 'e' || c == 'E' {
                        number.push(c);
                        chars.next();
                        current_pos += 1;
                        if let Some(&next_c) = chars.peek() {
                            if next_c == '+' || next_c == '-' {
                                number.push(next_c);
                                chars.next();
                                current_pos += 1;
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
                
                tokens.push(Token::new(token_type, number, start_pos));
                tokens_on_current_line += 1;
            }

            '"' => {
                chars.next();
                current_pos += 1;
                let mut string = String::new();
                let start_pos = current_pos;

                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next(); 
                        current_pos += 1;
                        break;
                    } else if c == '\n' {
                        current_line += 1;
                        string.push(c);
                        chars.next();
                        current_pos += 1;
                    } else {
                        string.push(c);
                        chars.next();
                        current_pos += 1;
                    }
                }

                tokens.push(Token::new(Tokentype::StringLiteral, string, start_pos));
                tokens_on_current_line += 1;
            }
            ':' => {
                chars.next();
                current_pos += 1;
                tokens.push(Token::new(Tokentype::Colon, ":".to_string(), token_start_pos));
                tokens_on_current_line += 1;
            }
            '+' => {
                chars.next();
                current_pos += 1;
                tokens.push(Token::new(Tokentype::Plus, "+".to_string(), token_start_pos));
                tokens_on_current_line += 1;
            }
            '-' => {
                chars.next();
                current_pos += 1;
                if chars.peek() == Some(&'>') {
                    chars.next();
                    current_pos += 1;
                    tokens.push(Token::new(Tokentype::Arrow, "->".to_string(), token_start_pos));
                } else {
                    tokens.push(Token::new(Tokentype::Minus, "-".to_string(), token_start_pos));
                }
                tokens_on_current_line += 1;
            }
            '*' => {
                chars.next();
                current_pos += 1;
                tokens.push(Token::new(Tokentype::Multiply, "*".to_string(), token_start_pos));
                tokens_on_current_line += 1;
            }
            '/' => {
                chars.next();
                current_pos += 1;
                if chars.peek() == Some(&'/') {
                    chars.next();
                    current_pos += 1;
                    
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            chars.next();
                            current_pos += 1;
                            if tokens_on_current_line > 0 {
                                line_tokens.push((current_line as u16, tokens_on_current_line as u16));
                            }
                            current_line += 1;
                            tokens_on_current_line = 0;
                            break;
                        }
                        chars.next();
                        current_pos += 1;
                    }
                } else if chars.peek() == Some(&'*') {
                    chars.next();
                    current_pos += 1;
                    
                    let mut nesting = 1;
                    while nesting > 0 {
                        if chars.peek().is_none()  {
                            break;
                        }
                        
                        let c = chars.peek().unwrap();
                        if *c == '\n' {
                            if tokens_on_current_line > 0 {
                                line_tokens.push((current_line as u16, tokens_on_current_line as u16));
                            }
                            current_line += 1;
                            tokens_on_current_line = 0;
                        }
                        
                        if chars.peek() == Some(&'*') {
                            chars.next();
                            current_pos += 1;
                            if chars.peek() == Some(&'/') {
                                chars.next();
                                current_pos += 1;
                                nesting -= 1;
                                continue;
                            }
                        } else if chars.peek() == Some(&'/') {
                            chars.next();
                            current_pos += 1;
                            if chars.peek() == Some(&'*') {
                                chars.next();
                                current_pos += 1;
                                nesting += 1;
                                continue;
                            }
                        } else {
                            chars.next();
                            current_pos += 1;
                        }
                    }
                } else {
                    tokens.push(Token::new(Tokentype::Divide, "/".to_string(), token_start_pos));
                    tokens_on_current_line += 1;
                }
            }
            '=' => {
                chars.next();
                current_pos += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    current_pos += 1;
                    tokens.push(Token::new(Tokentype::EqualEqual, "==".to_string(), token_start_pos));
                } else {
                    tokens.push(Token::new(Tokentype::Equal, "=".to_string(), token_start_pos));
                }
                tokens_on_current_line += 1;
            }
            '<' => {
                chars.next();
                current_pos += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    current_pos += 1;
                    tokens.push(Token::new(Tokentype::LessEqual, "<=".to_string(), token_start_pos));
                } else {
                    tokens.push(Token::new(Tokentype::Less, "<".to_string(), token_start_pos));
                }
                tokens_on_current_line += 1;
            }
            '>' => {
                chars.next();
                current_pos += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    current_pos += 1;
                    tokens.push(Token::new(Tokentype::GreaterEqual, ">=".to_string(), token_start_pos));
                } else {
                    tokens.push(Token::new(Tokentype::Greater, ">".to_string(), token_start_pos));
                }
                tokens_on_current_line += 1;
            }
            '!' => {
                chars.next();
                current_pos += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    current_pos += 1;
                    tokens.push(Token::new(Tokentype::NotEqual, "!=".to_string(), token_start_pos));
                } else {
                    tokens.push(Token::new(Tokentype::Not, "!".to_string(), token_start_pos));
                }
                tokens_on_current_line += 1;
            }
            ';' => {
                chars.next();
                current_pos += 1;
                tokens.push(Token::new(Tokentype::Semicolon, ";".to_string(), token_start_pos));
                tokens_on_current_line += 1;
            }
            '{' => {
                chars.next();
                current_pos += 1;
                tokens.push(Token::new(Tokentype::LeftBrace, "{".to_string(), token_start_pos));
                tokens_on_current_line += 1;
            }
            '}' => {
                chars.next();
                current_pos += 1;
                tokens.push(Token::new(Tokentype::RightBrace, "}".to_string(), token_start_pos));
                tokens_on_current_line += 1;
            }
            ',' => {
                chars.next();
                current_pos += 1;
                tokens.push(Token::new(Tokentype::Comma, ",".to_string(), token_start_pos));
                tokens_on_current_line += 1;
            }
            '(' => {
                chars.next();
                current_pos += 1;
                tokens.push(Token::new(Tokentype::LeftParen, "(".to_string(), token_start_pos));
                tokens_on_current_line += 1;
            }
            ')' => {
                chars.next();
                current_pos += 1;
                tokens.push(Token::new(Tokentype::RightParen, ")".to_string(), token_start_pos));
                tokens_on_current_line += 1;
            }
            '&' => {
                chars.next();
                current_pos += 1;
                if chars.peek() == Some(&'&') {
                    chars.next();
                    current_pos += 1;
                    tokens.push(Token::new(Tokentype::And, "&&".to_string(), token_start_pos));
                } else {
                    tokens.push(Token::new(Tokentype::Invalid, "&".to_string(), token_start_pos));
                }
                tokens_on_current_line += 1;
            }
            '|' => {
                chars.next();
                current_pos += 1;
                if chars.peek() == Some(&'|') {
                    chars.next();
                    current_pos += 1;
                    tokens.push(Token::new(Tokentype::Or, "||".to_string(), token_start_pos));
                } else {
                    tokens.push(Token::new(Tokentype::Invalid, "|".to_string(), token_start_pos));
                }
                tokens_on_current_line += 1;
            }
            _ => {
                let invalid_char = chars.next().unwrap();
                current_pos += 1;
                tokens.push(Token::new(Tokentype::Invalid, invalid_char.to_string(), token_start_pos));
                tokens_on_current_line += 1;
            }
        }
    }
    
    if tokens_on_current_line > 0 {
        line_tokens.push((current_line as u16, tokens_on_current_line as u16));
    }
    
    tokens.push(Token::new(Tokentype::Eof, "".to_string(), current_pos));
    
    let mut info = LineInfo::new(input);
    info.per_line = line_tokens;
    
    LexerResult {
        tokens,
        line_info: info,
    }
}
