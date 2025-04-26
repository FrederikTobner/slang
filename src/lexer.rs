use crate::token::{Token, Tokentype};

/// Converts source code text into a sequence of tokens
/// 
/// # Arguments
/// 
/// * `input` - The source code to tokenize
/// 
/// # Returns
/// 
/// A vector of tokens representing the source code
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_whitespace() => {
                chars.next();
            }

            c if c.is_alphabetic() => {
                let mut identifier = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match identifier.as_str() {
                    "let" => tokens.push(Token::new(Tokentype::Let, identifier)),
                    "struct" => tokens.push(Token::new(Tokentype::Struct, identifier)),
                    "fn" => tokens.push(Token::new(Tokentype::Fn, identifier)),
                    "return" => tokens.push(Token::new(Tokentype::Return, identifier)),
                    _ => tokens.push(Token::new(Tokentype::Identifier, identifier)),
                }
            }

            c if c.is_ascii_digit() => {
                let mut number = String::new();
                let mut is_float = false;
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        number.push(c);
                        chars.next();
                    } else if c == '.' {
                        if is_float {
                            break; 
                        }
                        is_float = true;
                        number.push(c);
                        chars.next();
                    } else if c == 'e' || c == 'E' {
                        number.push(c);
                        chars.next();
                        if let Some(&next_c) = chars.peek() {
                            if next_c == '+' || next_c == '-' {
                                number.push(next_c);
                                chars.next();
                            }
                        }
                    } else {
                        break;
                    } 
                }
                if is_float {
                    tokens.push(Token::new(Tokentype::FloatLiteral, number));
                } else {
                    tokens.push(Token::new(Tokentype::IntegerLiteral, number));
                }
            }

            '"' => {
                chars.next(); 
                let mut string = String::new();

                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next(); 
                        break;
                    } else {
                        string.push(c);
                        chars.next();
                    }
                }

                tokens.push(Token::new(Tokentype::StringLiteral, string));
            }
            ':' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Colon, ":".to_string()));
            }
            '+' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Plus, "+".to_string()));
            }
            '-' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::new(Tokentype::Arrow, "->".to_string()));
                } else {
                    tokens.push(Token::new(Tokentype::Minus, "-".to_string()));
                }
            }
            '*' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Multiply, "*".to_string()));
            }
            '/' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Divide, "/".to_string()));
            }
            '=' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Equal, "=".to_string()));
            }
            ';' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Semicolon, ";".to_string()));
            }
            '{' => {
                chars.next();
                tokens.push(Token::new(Tokentype::LeftBrace, "{".to_string()));
            }
            '}' => {
                chars.next();
                tokens.push(Token::new(Tokentype::RightBrace, "}".to_string()));
            }
            ',' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Comma, ",".to_string()));
            }
            '(' => {
                chars.next();
                tokens.push(Token::new(Tokentype::LeftParen, "(".to_string()));
            }
            ')' => {
                chars.next();
                tokens.push(Token::new(Tokentype::RightParen, ")".to_string()));
            }
            _ => {
                let invalid_char = chars.next().unwrap();
                tokens.push(Token::new(Tokentype::Invalid, invalid_char.to_string()));
            }
        }
    }
    
    tokens.push(Token::new(Tokentype::Eof, "".to_string()));
    
    tokens
}
