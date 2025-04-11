#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tokentype {
    Identifier,     // x, y, myVar
    IntegerLiteral, // 123
    StringLiteral,  // "string"
    Let,            // let
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Invalid,        // Unrecognized token
    Equal,          // =
    Colon,          // :     
    TypeInt,        // Int
    TypeString,     // String
    Semicolon,      // ;
}

#[derive(Debug)]
pub struct Token {
    pub token_type: Tokentype,
    pub value: String,
}

impl Token {
    pub fn new(token_type: Tokentype, value: String) -> Token {
        Token { token_type, value }
    }
}
