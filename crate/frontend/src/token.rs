use std::fmt::Display;

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
