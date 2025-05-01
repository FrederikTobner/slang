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

/// Represents a token in the source code
#[derive(Debug)]
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
