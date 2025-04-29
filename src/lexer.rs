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
                    "true" => tokens.push(Token::new(Tokentype::BooleanLiteral, identifier)),
                    "false" => tokens.push(Token::new(Tokentype::BooleanLiteral, identifier)),
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
                    let token_type = Tokentype::FloatLiteral;                
                    tokens.push(Token::new(token_type, number));
                } else {
                    let token_type = Tokentype::IntegerLiteral;
                    tokens.push(Token::new(token_type, number));
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
                if chars.peek() == Some(&'/') {
                    // Single-line comment
                    chars.next(); // Consume the second '/'
                    
                    // Skip all characters until the end of the line or end of file
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            chars.next(); // Consume the newline
                            break;
                        }
                        chars.next(); // Consume the current character
                    }
                } else if chars.peek() == Some(&'*') {
                    // Multi-line comment
                    chars.next(); // Consume the '*'
                    
                    // Loop until we find the closing '*/' or reach end of file
                    let mut nesting = 1;
                    while nesting > 0 {
                        if chars.peek() == None {
                            // Reached EOF without closing the comment
                            break;
                        }
                        
                        if chars.peek() == Some(&'*') {
                            chars.next(); // Consume the '*'
                            if chars.peek() == Some(&'/') {
                                chars.next(); // Consume the '/'
                                nesting -= 1;
                                continue;
                            }
                        } else if chars.peek() == Some(&'/') {
                            chars.next(); // Consume the '/'
                            if chars.peek() == Some(&'*') {
                                chars.next(); // Consume the '*'
                                nesting += 1;
                                continue;
                            }
                        } else {
                            chars.next(); // Consume the current character
                        }
                    }
                } else {
                    tokens.push(Token::new(Tokentype::Divide, "/".to_string()));
                }
            }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new(Tokentype::EqualEqual, "==".to_string()));
                } else {
                    tokens.push(Token::new(Tokentype::Equal, "=".to_string()));
                }
            }
            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new(Tokentype::LessEqual, "<=".to_string()));
                } else {
                    tokens.push(Token::new(Tokentype::Less, "<".to_string()));
                }
            }
            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new(Tokentype::GreaterEqual, ">=".to_string()));
                } else {
                    tokens.push(Token::new(Tokentype::Greater, ">".to_string()));
                }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new(Tokentype::NotEqual, "!=".to_string()));
                } else {
                    tokens.push(Token::new(Tokentype::Not, "!".to_string()));
                }
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
            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    chars.next();
                    tokens.push(Token::new(Tokentype::And, "&&".to_string()));
                } else {
                    tokens.push(Token::new(Tokentype::Invalid, "&".to_string()));
                }
            }
            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(Token::new(Tokentype::Or, "||".to_string()));
                } else {
                    tokens.push(Token::new(Tokentype::Invalid, "|".to_string()));
                }
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
