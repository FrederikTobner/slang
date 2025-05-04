use std::fmt::Display;
use colored::Colorize;

/// Types of tokens in the language lexer
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tokentype {
    Identifier,     // x, y, myVar
    IntegerLiteral, // 123
    FloatLiteral,    // 123.45
    StringLiteral,  // "hello world"
    BooleanLiteral, // true, false
    Let,            // let
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Not,            // !
    And,            // &&
    Or,             // ||
    Greater,        // >
    Less,           // <
    GreaterEqual,   // >=
    LessEqual,      // <=
    EqualEqual,     // ==
    NotEqual,       // !=
    Invalid,        // Unrecognized token
    Equal,          // =
    Colon,          // :     
    Semicolon,      // ;
    Struct,         // struct
    LeftBrace,      // {
    RightBrace,     // }
    Comma,          // ,
    Fn,             // fn
    LeftParen,      // (
    RightParen,     // )
    Arrow,          // ->
    Return,         // return
    
    Eof,            // End of file
}

impl Display for Tokentype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tokentype::Identifier => "identifier",
            Tokentype::IntegerLiteral => "integer literal",
            Tokentype::FloatLiteral => "float literal",
            Tokentype::StringLiteral => "string literal",
            Tokentype::BooleanLiteral => "boolean literal",
            Tokentype::Let => "let keyword",
            Tokentype::Plus => "'+'",
            Tokentype::Minus => "'-'",
            Tokentype::Multiply => "'*'",
            Tokentype::Divide => "'/'",
            Tokentype::Not => "'!'",
            Tokentype::And => "'&&'",
            Tokentype::Or => "'||'",
            Tokentype::Greater => "'>'",
            Tokentype::Less => "'<'",
            Tokentype::GreaterEqual => "'>='",
            Tokentype::LessEqual => "'<='",
            Tokentype::EqualEqual => "'=='",
            Tokentype::NotEqual => "'!='",
            Tokentype::Invalid => "invalid token",
            Tokentype::Equal => "'='",
            Tokentype::Colon => "':'",     
            Tokentype::Semicolon => "';'",
            Tokentype::Struct => "sturct keyword",
            Tokentype::LeftBrace => "'{'",
            Tokentype::RightBrace => "'}'",
            Tokentype::Comma => "','",
            Tokentype::Fn => "fn keyword",
            Tokentype::LeftParen => "'('",
            Tokentype::RightParen => "')'",
            Tokentype::Arrow => "'->'",
            Tokentype::Return => "return keyword", 
            
            // End of file
            Tokentype::Eof  => "<EOF>",
        })
    }
}

/// Represents a token in the source code
pub struct Token {
    /// The type of the token
    pub token_type: Tokentype,
    /// The actual text of the token
    pub lexeme: String,
    /// Position index - used with LineInfo to determine line number
    pub pos: usize,
}

impl Token {

    /// Creates a new token with the given type, lexeme, and position
    pub fn new(token_type: Tokentype, lexeme: String, pos: usize) -> Token {
        Token { token_type, lexeme, pos }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.token_type, self.lexeme)
    }
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
    pub fn format_error(&self, pos: usize, message: &str, token_length: usize) -> String {
        let (line, col) = self.get_line_col(pos);
        let line_str = self.get_line_text(line).unwrap_or("");
        
        format!(
            "Error at line {}, column {}: {}\n{}\n{}{}",
            line, col, message, line_str,
            " ".repeat(col - 1), "^".repeat(token_length).red()
        )
    }
}
