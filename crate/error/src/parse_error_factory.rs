use crate::ErrorCode;
use crate::parse_error::ParseError;
use crate::Location;

/// Factory for creating specific parse errors with clear, intention-revealing methods.
/// Each method creates a semantically meaningful error with appropriate error codes.
pub struct ParseErrorFactory;

impl ParseErrorFactory {
    /// Create an "expected else after if" error
    pub fn expected_else_after_if(location: Location) -> ParseError {
        ParseError::InvalidSyntax {
            message: "Expected 'else' after if expression".to_owned(),
            suggestion: None,
            location,
            error_code: Some(ErrorCode::ExpectedElse),
        }
    }

    /// Create an "expected semicolon" error with optional context
    pub fn expected_semicolon(location: Location, context: Option<&str>) -> ParseError {
        let message = match context {
            Some(ctx) => format!("Expected ';' {ctx}"),
            None => "Expected ';'".to_owned(),
        };
        
        ParseError::InvalidSyntax {
            message,
            suggestion: None,
            location,
            error_code: Some(ErrorCode::ExpectedSemicolon),
        }
    }

    /// Create an "expected equals" error
    pub fn expected_equals(location: Location) -> ParseError {
        ParseError::InvalidSyntax {
            message: "Expected '='".to_owned(),
            suggestion: None,
            location,
            error_code: Some(ErrorCode::ExpectedEquals),
        }
    }

    /// Create an "expected opening parenthesis" error with optional context
    pub fn expected_opening_paren(location: Location, context: Option<&str>) -> ParseError {
        let message = match context {
            Some(ctx) => format!(" Expected '(' {ctx}"),
            None => " Expected '('".to_owned(),
        };
        
        ParseError::InvalidSyntax {
            message,
            suggestion: None,
            location,
            error_code: Some(ErrorCode::ExpectedOpeningParen),
        }
    }

    /// Create an "expected closing parenthesis" error with optional context
    pub fn expected_closing_paren(location: Location, context: Option<&str>) -> ParseError {
        let message = match context {
            Some(ctx) => format!(" Expected ')' {ctx}"),
            None => " Expected ')'".to_owned(),
        };
        
        ParseError::InvalidSyntax {
            message,
            suggestion: None,
            location,
            error_code: Some(ErrorCode::ExpectedClosingParen),
        }
    }

    /// Create an "expected opening brace" error with optional context
    pub fn expected_opening_brace(location: Location, context: Option<&str>) -> ParseError {
        let message = match context {
            Some(ctx) => format!("Expected '{{' {ctx}"),
            None => "Expected '{{'".to_owned(),
        };
        
        ParseError::InvalidSyntax {
            message,
            suggestion: None,
            location,
            error_code: Some(ErrorCode::ExpectedOpeningBrace),
        }
    }

    /// Create an "expected closing brace" error with optional context
    pub fn expected_closing_brace(location: Location, context: Option<&str>) -> ParseError {
        let message = match context {
            Some(ctx) => format!("Expected '}}' {ctx}"),
            None => "Expected '}}'".to_owned(),
        };
        
        ParseError::InvalidSyntax {
            message,
            suggestion: None,
            location,
            error_code: Some(ErrorCode::ExpectedClosingBrace),
        }
    }

    /// Create an "expected colon" error
    pub fn expected_colon(location: Location) -> ParseError {
        ParseError::InvalidSyntax {
            message: "Expected ':'".to_owned(),
            suggestion: None,
            location,
            error_code: Some(ErrorCode::ExpectedColon),
        }
    }

    /// Create an "expected comma" error with optional context
    pub fn expected_comma(location: Location, context: Option<&str>) -> ParseError {
        let message = match context {
            Some(ctx) => format!("Expected ',' {ctx}"),
            None => "Expected ','".to_owned(),
        };
        
        ParseError::InvalidSyntax {
            message,
            suggestion: None,
            location,
            error_code: Some(ErrorCode::ExpectedComma),
        }
    }

    /// Create an "expected identifier" error with optional context description
    pub fn expected_identifier(location: Location, description: Option<&str>) -> ParseError {
        let message = match description {
            Some(desc) => format!(" Expected {desc}"),
            None => " Expected identifier".to_owned(),
        };
        
        ParseError::InvalidSyntax {
            message,
            suggestion: None,
            location,
            error_code: Some(ErrorCode::ExpectedIdentifier),
        }
    }

    /// Create an "invalid number literal" error
    pub fn invalid_number_literal(location: Location, value: &str, reason: &str) -> ParseError {
        ParseError::InvalidNumber {
            value: value.to_owned(),
            reason: reason.to_owned(),
            location,
            error_code: Some(ErrorCode::InvalidNumberLiteral),
        }
    }

    /// Create an "unknown type" error
    pub fn unknown_type(location: Location, message: &str) -> ParseError {
        ParseError::InvalidSyntax {
            message: message.to_owned(),
            suggestion: Some("Check available type names".to_owned()),
            location,
            error_code: Some(ErrorCode::UnknownType),
        }
    }

    /// Create a "value out of range" error
    pub fn value_out_of_range(location: Location, value: &str, reason: &str) -> ParseError {
        ParseError::InvalidNumber {
            value: value.to_owned(),
            reason: reason.to_owned(),
            location,
            error_code: Some(ErrorCode::ValueOutOfRange),
        }
    }

    /// Create a generic "invalid syntax" error with optional suggestion
    pub fn invalid_syntax(location: Location, message: &str, suggestion: Option<&str>) -> ParseError {
        ParseError::InvalidSyntax {
            message: message.to_owned(),
            suggestion: suggestion.map(|s| s.to_owned()),
            location,
            error_code: Some(ErrorCode::InvalidSyntax),
        }
    }
}
