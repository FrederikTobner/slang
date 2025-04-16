use crate::token::{Token, Tokentype};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Skip whitespace
            c if c.is_whitespace() => {
                chars.next();
            }

            // Identifiers and keywords
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
                    _ => tokens.push(Token::new(Tokentype::Identifier, identifier)),
                }
            }

            // Integer literals
            c if c.is_digit(10) => {
                let mut number = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::new(Tokentype::IntegerLiteral, number));
            }

            // String literals
            '"' => {
                chars.next(); // Skip opening quote
                let mut string = String::new();

                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next(); // Skip closing quote
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
            // Operators
            '+' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Plus, "+".to_string()));
            }
            '-' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Minus, "-".to_string()));
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

            // Invalid characters
            _ => {
                let invalid_char = chars.next().unwrap();
                tokens.push(Token::new(Tokentype::Invalid, invalid_char.to_string()));
            }
        }
    }
    tokens
}
