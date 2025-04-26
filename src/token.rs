/// Types of tokens in the language lexer
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tokentype {
    Identifier,     // x, y, myVar
    IntegerLiteral, // 123
    FloatLiteral,    // 123.45
    StringLiteral,  // "hello world"
    Let,            // let
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
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
}

impl Token {
    /// Creates a new token with the given type and lexeme
    pub fn new(token_type: Tokentype, lexeme: String) -> Token {
        Token { token_type, lexeme }
    }
}
