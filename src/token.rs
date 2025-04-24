#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tokentype {
    Identifier,     // x, y, myVar
    IntegerLiteral, // 123
    FloatLiteral,    // 123.45
    StringLiteral,  // "string"
    Let,            // let
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Invalid,        // Unrecognized token
    Equal,          // =
    Colon,          // :     
    Semicolon,      // ;

    Struct,
    LeftBrace,
    RightBrace,
    Comma,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: Tokentype,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: Tokentype, lexeme: String) -> Token {
        Token { token_type, lexeme }
    }
}
