/// Comprehensive error codes for all compilation errors in the Slang compiler.
/// 
/// This enum provides unified error codes and descriptions for both parsing and semantic analysis errors.
/// Each variant maps to a unique u16 code and has an associated description.
/// 
/// Error code ranges:
/// - 1000-1999: Parse errors (syntax and structural issues)
/// - 2000-2999: Semantic analysis errors (type checking, scope resolution)
/// - 3000-3999: Generic compile errors (not specifically categorized)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // Parse Errors (1000-1999)

    /// Expected a semicolon after a statement
    ExpectedSemicolon = 1001,
    /// Expected a closing brace
    ExpectedClosingBrace = 1002,
    /// Expected a closing parenthesis
    ExpectedClosingParen = 1003,
    /// Expected a closing bracket
    ExpectedClosingBracket = 1004,
    /// Expected an opening brace
    ExpectedOpeningBrace = 1005,
    /// Expected an opening parenthesis
    ExpectedOpeningParen = 1006,
    /// Expected an identifier
    ExpectedIdentifier = 1007,
    /// Expected a type annotation
    ExpectedType = 1008,
    /// Expected an expression
    ExpectedExpression = 1009,
    /// Expected a statement
    ExpectedStatement = 1010,
    /// Expected a function parameter
    ExpectedParameter = 1011,
    /// Expected an assignment operator
    ExpectedAssignment = 1012,
    /// Expected a comma separator
    ExpectedComma = 1013,
    /// Expected a colon
    ExpectedColon = 1014,
    /// Expected an equals sign
    ExpectedEquals = 1015,
    /// Expected a function body
    ExpectedFunctionBody = 1016,
    /// Expected a struct field
    ExpectedStructField = 1017,
    /// Expected end of file
    ExpectedEof = 1018,
    /// Unexpected token encountered
    UnexpectedToken = 1019,
    /// Invalid number literal format
    InvalidNumberLiteral = 1020,
    /// Invalid string literal format
    InvalidStringLiteral = 1021,
    /// Invalid character literal format
    InvalidCharLiteral = 1022,
    /// Invalid escape sequence in string
    InvalidEscapeSequence = 1023,
    /// Unterminated string literal
    UnterminatedString = 1024,
    /// Unterminated character literal
    UnterminatedChar = 1025,
    /// Malformed comment
    MalformedComment = 1026,
    /// Invalid token in input
    InvalidToken = 1027,
    /// Nested function definitions not allowed
    NestedFunction = 1028,
    /// Invalid syntax in expression
    InvalidSyntax = 1029,
    /// Encountered unknow type during parsing
    UnknownType = 1030,
    /// Expected 'else' keyword after if expression
    ExpectedElse = 1031,

    // Semantic Analysis Errors (2000-2999)

    /// Variable used before being defined
    UndefinedVariable = 2001,
    /// Variable redefinition in the same scope
    VariableRedefinition = 2002,
    /// Symbol redefinition conflict
    SymbolRedefinition = 2003,
    /// Invalid type for struct field
    InvalidFieldType = 2004,
    /// Type mismatch between expected and actual types
    TypeMismatch = 2005,
    /// Incompatible types for operation
    OperationTypeMismatch = 2006,
    /// Logical operator used with non-boolean types
    LogicalOperatorTypeMismatch = 2007,
    /// Value out of range for target type
    ValueOutOfRange = 2008,
    /// Function called with wrong number of arguments
    ArgumentCountMismatch = 2009,
    /// Function argument has wrong type
    ArgumentTypeMismatch = 2010,
    /// Return statement used outside function
    ReturnOutsideFunction = 2011,
    /// Return type doesn't match function declaration
    ReturnTypeMismatch = 2012,
    /// Missing return value for non-void function
    MissingReturnValue = 2013,
    /// Function called but not defined
    UndefinedFunction = 2014,
    /// Invalid unary operation for type
    InvalidUnaryOperation = 2015,
    /// Assignment to immutable variable
    AssignmentToImmutableVariable = 2016,
    /// Expression has invalid form or context
    InvalidExpression = 2017,

    // Generic Compile Errors (3000-3999)

    /// Generic compile error not categorized
    GenericCompileError = 3000,
}

impl ErrorCode {
    /// Get the numeric error code as a u16
    pub fn code(&self) -> u16 {
        *self as u16
    }

    /// Get a short description of the error
    pub fn description(&self) -> &'static str {
        match self {
            ErrorCode::ExpectedSemicolon => "Expected semicolon after statement",
            ErrorCode::ExpectedClosingBrace => "Expected closing brace '}'",
            ErrorCode::ExpectedClosingParen => "Expected closing parenthesis ')'",
            ErrorCode::ExpectedClosingBracket => "Expected closing bracket ']'",
            ErrorCode::ExpectedOpeningBrace => "Expected opening brace '{'",
            ErrorCode::ExpectedOpeningParen => "Expected opening parenthesis '('",
            ErrorCode::ExpectedIdentifier => "Expected identifier",
            ErrorCode::ExpectedType => "Expected type annotation",
            ErrorCode::ExpectedExpression => "Expected expression",
            ErrorCode::ExpectedStatement => "Expected statement",
            ErrorCode::ExpectedParameter => "Expected function parameter",
            ErrorCode::ExpectedAssignment => "Expected assignment operator",
            ErrorCode::ExpectedComma => "Expected comma separator",
            ErrorCode::ExpectedColon => "Expected colon ':'",
            ErrorCode::ExpectedEquals => "Expected equals sign '='",
            ErrorCode::ExpectedFunctionBody => "Expected function body",
            ErrorCode::ExpectedStructField => "Expected struct field",
            ErrorCode::ExpectedEof => "Expected end of file",
            ErrorCode::UnexpectedToken => "Unexpected token",
            ErrorCode::InvalidNumberLiteral => "Invalid number literal format",
            ErrorCode::InvalidStringLiteral => "Invalid string literal format",
            ErrorCode::InvalidCharLiteral => "Invalid character literal format",
            ErrorCode::InvalidEscapeSequence => "Invalid escape sequence",
            ErrorCode::UnterminatedString => "Unterminated string literal",
            ErrorCode::UnterminatedChar => "Unterminated character literal",
            ErrorCode::MalformedComment => "Malformed comment",
            ErrorCode::InvalidToken => "Invalid token",
            ErrorCode::NestedFunction => "Nested function definitions not allowed",
            ErrorCode::InvalidSyntax => "Invalid syntax",
            ErrorCode::UnknownType => "Unknow type",
            ErrorCode::ExpectedElse => "Expected 'else' after if expression",

            // Semantic Analysis Errors
            ErrorCode::UndefinedVariable => "Undefined variable",
            ErrorCode::VariableRedefinition => "Variable already defined",
            ErrorCode::SymbolRedefinition => "Symbol redefinition",
            ErrorCode::InvalidFieldType => "Invalid field type",
            ErrorCode::TypeMismatch => "Type mismatch",
            ErrorCode::OperationTypeMismatch => "Incompatible types for operation",
            ErrorCode::LogicalOperatorTypeMismatch => "Logical operator requires boolean types",
            ErrorCode::ValueOutOfRange => "Value out of range for type",
            ErrorCode::ArgumentCountMismatch => "Wrong number of function arguments",
            ErrorCode::ArgumentTypeMismatch => "Function argument type mismatch",
            ErrorCode::ReturnOutsideFunction => "Return statement outside function",
            ErrorCode::ReturnTypeMismatch => "Return type mismatch",
            ErrorCode::MissingReturnValue => "Missing return value",
            ErrorCode::UndefinedFunction => "Undefined function",
            ErrorCode::InvalidUnaryOperation => "Invalid unary operation for type",
            ErrorCode::AssignmentToImmutableVariable => "Assignment to immutable variable",
            ErrorCode::InvalidExpression => "Invalid expression",
            ErrorCode::GenericCompileError => "Generic compile error",
        }
    }

    /// Check if this is a parse error (1000-1999 range)
    pub fn is_parse_error(&self) -> bool {
        let code = self.code();
        code >= 1000 && code < 2000
    }

    /// Check if this is a semantic error (2000-2999 range)
    pub fn is_semantic_error(&self) -> bool {
        let code = self.code();
        code >= 2000 && code < 3000
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[E{:04}]", self.code())
    }
}
