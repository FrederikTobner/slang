use slang_error::{
    DomainError, ErrorCategory, ErrorCode, FunctionCallErrorKind, Location, SemanticError,
};

fn location() -> Location {
    Location::new_simple(12, 3, 4)
}

fn function_call_error(error_kind: FunctionCallErrorKind) -> SemanticError {
    SemanticError::FunctionCallError {
        function_name: "foo".to_string(),
        error_kind,
        location: location(),
    }
}

#[test]
fn undefined_symbol_without_suggestions() {
    let error = SemanticError::UndefinedSymbol {
        name: "x".to_string(),
        suggestions: vec![],
        location: location(),
    };
    let compiler_error = error.to_compiler_error();

    assert_eq!(compiler_error.error_code, ErrorCode::UndefinedVariable);
    assert_eq!(compiler_error.message, "Undefined symbol: x");
    assert_eq!(compiler_error.line, 3);
    assert_eq!(compiler_error.column, 4);
    assert_eq!(compiler_error.position, 12);
    assert_eq!(error.short_description(), "Undefined symbol: x");
    assert_eq!(error.to_string(), "Undefined symbol: x");
}

#[test]
fn undefined_symbol_with_suggestions() {
    let error = SemanticError::UndefinedSymbol {
        name: "x".to_string(),
        suggestions: vec!["y".to_string(), "z".to_string()],
        location: location(),
    };

    assert_eq!(
        error.to_compiler_error().message,
        "Undefined symbol: x. Did you mean: y, z?"
    );
}

#[test]
fn symbol_redefinition() {
    let original_location = Location::new_simple(1, 1, 1);
    let redefinition_location = Location::new_simple(30, 5, 2);
    let error = SemanticError::SymbolRedefinition {
        name: "x".to_string(),
        kind: "variable".to_string(),
        original_location,
        redefinition_location,
    };
    let compiler_error = error.to_compiler_error();

    assert_eq!(compiler_error.error_code, ErrorCode::SymbolRedefinition);
    assert_eq!(
        compiler_error.message,
        "variable 'x' is already defined in the current scope"
    );
    assert_eq!(error.location().position, 30);
    assert_eq!(error.short_description(), "Symbol redefinition: x");
}

#[test]
fn type_mismatch_variants() {
    let build = |context: &str, can_convert: bool| SemanticError::TypeMismatch {
        expected: "i32".to_string(),
        found: "f64".to_string(),
        context: context.to_string(),
        can_convert,
        location: location(),
    };

    let plain = build("", false).to_compiler_error();
    assert_eq!(plain.error_code, ErrorCode::TypeMismatch);
    assert_eq!(plain.message, "Type mismatch: expected i32, found f64 ");

    let with_context = build("in assignment", false).to_compiler_error();
    assert_eq!(
        with_context.message,
        "Type mismatch: expected i32, found f64 (in assignment)"
    );

    let convertible = build("in assignment", true).to_compiler_error();
    assert!(
        convertible
            .message
            .ends_with("Consider explicit type conversion.")
    );
    assert_eq!(build("", false).short_description(), "Type mismatch");
}

#[test]
fn invalid_operation() {
    let build = |suggestion: Option<&str>| SemanticError::InvalidOperation {
        operation: "+".to_string(),
        left_type: "i32".to_string(),
        right_type: "string".to_string(),
        suggestion: suggestion.map(str::to_string),
        location: location(),
    };

    let without = build(None);
    assert_eq!(
        without.to_compiler_error().message,
        "Invalid operation '+' between i32 and string"
    );
    assert_eq!(
        without.to_compiler_error().error_code,
        ErrorCode::OperationTypeMismatch
    );
    assert_eq!(without.short_description(), "Invalid operation: +");

    let with = build(Some("Cast one operand."));
    assert_eq!(
        with.to_compiler_error().message,
        "Invalid operation '+' between i32 and string. Cast one operand."
    );
}

#[test]
fn function_call_undefined_function() {
    let without = function_call_error(FunctionCallErrorKind::UndefinedFunction {
        suggestions: vec![],
    });
    assert_eq!(
        without.to_compiler_error().message,
        "Undefined function: foo"
    );
    assert_eq!(
        without.to_compiler_error().error_code,
        ErrorCode::UndefinedFunction
    );
    assert_eq!(without.short_description(), "Function call error: foo");

    let with = function_call_error(FunctionCallErrorKind::UndefinedFunction {
        suggestions: vec!["bar".to_string()],
    });
    assert_eq!(
        with.to_compiler_error().message,
        "Undefined function: foo. Did you mean: bar?"
    );
}

#[test]
fn function_call_argument_count_mismatch() {
    let error = function_call_error(FunctionCallErrorKind::ArgumentCountMismatch {
        expected: 2,
        found: 3,
    });

    assert_eq!(
        error.to_compiler_error().message,
        "Function 'foo' expects 2 arguments, but 3 were provided"
    );
}

#[test]
fn function_call_argument_type_mismatch() {
    let build = |can_convert| {
        function_call_error(FunctionCallErrorKind::ArgumentTypeMismatch {
            argument_index: 1,
            expected: "i32".to_string(),
            found: "f64".to_string(),
            can_convert,
        })
    };

    assert_eq!(
        build(false).to_compiler_error().message,
        "Argument 2 of function 'foo': expected i32, found f64"
    );
    assert_eq!(
        build(true).to_compiler_error().message,
        "Argument 2 of function 'foo': expected i32, found f64. Consider explicit type conversion."
    );
}

#[test]
fn category_is_semantic() {
    let error = function_call_error(FunctionCallErrorKind::ArgumentCountMismatch {
        expected: 0,
        found: 1,
    });

    assert_eq!(error.category(), ErrorCategory::Semantic);
}
