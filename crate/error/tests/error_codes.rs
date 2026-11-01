use slang_error::ErrorCode;

const PARSE_CODES: [ErrorCode; 37] = [
    ErrorCode::ExpectedSemicolon,
    ErrorCode::ExpectedClosingBrace,
    ErrorCode::ExpectedClosingParen,
    ErrorCode::ExpectedClosingBracket,
    ErrorCode::ExpectedOpeningBrace,
    ErrorCode::ExpectedOpeningParen,
    ErrorCode::ExpectedIdentifier,
    ErrorCode::ExpectedType,
    ErrorCode::ExpectedExpression,
    ErrorCode::ExpectedStatement,
    ErrorCode::ExpectedParameter,
    ErrorCode::ExpectedAssignment,
    ErrorCode::ExpectedComma,
    ErrorCode::ExpectedColon,
    ErrorCode::ExpectedEquals,
    ErrorCode::ExpectedFunctionBody,
    ErrorCode::ExpectedStructField,
    ErrorCode::ExpectedEof,
    ErrorCode::UnexpectedToken,
    ErrorCode::InvalidNumberLiteral,
    ErrorCode::InvalidStringLiteral,
    ErrorCode::InvalidCharLiteral,
    ErrorCode::InvalidEscapeSequence,
    ErrorCode::UnterminatedString,
    ErrorCode::UnterminatedChar,
    ErrorCode::MalformedComment,
    ErrorCode::InvalidToken,
    ErrorCode::NestedFunction,
    ErrorCode::InvalidSyntax,
    ErrorCode::UnknownType,
    ErrorCode::ExpectedElse,
    ErrorCode::ExpectedClosingQuote,
    ErrorCode::ExpectedToken,
    ErrorCode::UnexpectedEof,
    ErrorCode::InvalidNumber,
    ErrorCode::SyntaxError,
    ErrorCode::MismatchedDelimiters,
];

const SEMANTIC_CODES: [ErrorCode; 25] = [
    ErrorCode::UndefinedVariable,
    ErrorCode::VariableRedefinition,
    ErrorCode::SymbolRedefinition,
    ErrorCode::InvalidFieldType,
    ErrorCode::TypeMismatch,
    ErrorCode::OperationTypeMismatch,
    ErrorCode::LogicalOperatorTypeMismatch,
    ErrorCode::ValueOutOfRange,
    ErrorCode::ArgumentCountMismatch,
    ErrorCode::ArgumentTypeMismatch,
    ErrorCode::ReturnOutsideFunction,
    ErrorCode::ReturnTypeMismatch,
    ErrorCode::MissingReturnValue,
    ErrorCode::UndefinedFunction,
    ErrorCode::InvalidUnaryOperation,
    ErrorCode::AssignmentToImmutableVariable,
    ErrorCode::InvalidExpression,
    ErrorCode::VariableNotCallable,
    ErrorCode::UndefinedType,
    ErrorCode::CircularDependency,
    ErrorCode::InvalidType,
    ErrorCode::StackOverflow,
    ErrorCode::TooManyConstants,
    ErrorCode::UnsupportedFeature,
    ErrorCode::InternalError,
];

#[test]
fn numeric_codes() {
    assert_eq!(ErrorCode::ExpectedSemicolon.code(), 1001);
    assert_eq!(ErrorCode::MismatchedDelimiters.code(), 1037);
    assert_eq!(ErrorCode::UndefinedVariable.code(), 2001);
    assert_eq!(ErrorCode::InternalError.code(), 2503);
    assert_eq!(ErrorCode::GenericCompileError.code(), 3000);
}

#[test]
fn display_format() {
    assert_eq!(ErrorCode::ExpectedSemicolon.to_string(), "[E1001]");
    assert_eq!(ErrorCode::GenericCompileError.to_string(), "[E3000]");
}

#[test]
fn parse_errors_are_classified() {
    for code in PARSE_CODES {
        assert!(code.is_parse_error(), "{code:?} should be a parse error");
        assert!(!code.is_semantic_error(), "{code:?} is not semantic");
    }
}

#[test]
fn semantic_errors_are_classified() {
    for code in SEMANTIC_CODES {
        assert!(code.is_semantic_error(), "{code:?} should be semantic");
        assert!(!code.is_parse_error(), "{code:?} is not a parse error");
    }
}

#[test]
fn generic_error_is_neither_parse_nor_semantic() {
    assert!(!ErrorCode::GenericCompileError.is_parse_error());
    assert!(!ErrorCode::GenericCompileError.is_semantic_error());
}

#[test]
fn every_code_has_a_description() {
    for code in PARSE_CODES
        .iter()
        .chain(SEMANTIC_CODES.iter())
        .chain([ErrorCode::GenericCompileError].iter())
    {
        assert!(
            !code.description().is_empty(),
            "{code:?} has no description"
        );
    }
}

#[test]
fn descriptions() {
    assert_eq!(
        ErrorCode::ExpectedSemicolon.description(),
        "Expected semicolon after statement"
    );
    assert_eq!(ErrorCode::TypeMismatch.description(), "Type mismatch");
}
