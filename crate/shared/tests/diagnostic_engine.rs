use slang_error::{CompilationError, ErrorCode, Location};
use slang_shared::diagnostic_engine::ErrorSeverity;
use slang_shared::{Diagnostic, DiagnosticEngine, Suggestion};

fn location() -> Location {
    Location::new(4, 1, 5, 3)
}

fn diagnostic(severity: ErrorSeverity) -> Diagnostic {
    Diagnostic {
        severity,
        error_code: ErrorCode::TypeMismatch,
        message: "message".to_string(),
        location: location(),
        suggestions: Vec::new(),
        related: Vec::new(),
    }
}

#[test]
fn new_engine_is_empty() {
    // Arrange
    let engine = DiagnosticEngine::new();

    // Act & Assert
    assert!(!engine.has_errors());
    assert_eq!(engine.error_count(), 0);
    assert_eq!(engine.warning_count(), 0);
    assert!(!engine.is_recovery_mode());
    assert!(engine.finish().is_ok());
}

#[test]
fn emit_error_and_warning_are_counted() {
    // Arrange
    let mut engine = DiagnosticEngine::default();

    // Act
    engine.emit_error(ErrorCode::UndefinedVariable, "bad".to_string(), location());
    engine.emit_warning(ErrorCode::InvalidExpression, "odd".to_string(), location());
    engine.emit_warning(ErrorCode::InvalidExpression, "odd".to_string(), location());

    // Assert
    assert!(engine.has_errors());
    assert_eq!(engine.error_count(), 1);
    assert_eq!(engine.warning_count(), 2);
}

#[test]
fn notes_are_not_counted() {
    // Arrange
    let mut engine = DiagnosticEngine::new();

    // Act
    engine.emit(diagnostic(ErrorSeverity::Note));

    // Assert
    assert_eq!(engine.error_count(), 0);
    assert_eq!(engine.warning_count(), 0);
    assert_eq!(engine.into_errors().len(), 1);
}

#[test]
fn emit_with_suggestion_keeps_suggestion() {
    // Arrange
    let mut engine = DiagnosticEngine::new();
    let suggestion = Suggestion {
        message: "add a semicolon".to_string(),
        replacement: Some(";".to_string()),
        location: Some(location()),
    };

    // Act
    engine.emit_with_suggestion(
        ErrorCode::ExpectedSemicolon,
        "missing".to_string(),
        location(),
        suggestion,
    );

    // Assert
    let diagnostics = engine.into_errors();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].suggestions.len(), 1);
    assert_eq!(diagnostics[0].suggestions[0].message, "add a semicolon");
}

#[test]
fn emit_compiler_error_round_trips() {
    // Arrange
    let mut engine = DiagnosticEngine::new();
    let error = CompilationError::new(
        ErrorCode::ExpectedSemicolon,
        "missing".to_string(),
        2,
        7,
        15,
        None,
    );

    // Act
    engine.emit_compiler_error(error);
    let errors = engine.get_compiler_errors();

    // Assert
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].error_code, ErrorCode::ExpectedSemicolon);
    assert_eq!(errors[0].message, "missing");
    assert_eq!(errors[0].line, 2);
    assert_eq!(errors[0].column, 7);
    assert_eq!(errors[0].position, 15);
    assert_eq!(errors[0].token_length, Some(1));
}

#[test]
fn get_compiler_errors_skips_warnings() {
    // Arrange
    let mut engine = DiagnosticEngine::new();
    engine.emit_warning(ErrorCode::InvalidExpression, "odd".to_string(), location());
    engine.emit_error(ErrorCode::UndefinedVariable, "bad".to_string(), location());

    // Act
    let errors = engine.get_compiler_errors();

    // Assert
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message, "bad");
}

#[test]
fn finish_returns_diagnostics_on_error() {
    // Arrange
    let mut engine = DiagnosticEngine::new();
    engine.emit_error(ErrorCode::UndefinedVariable, "bad".to_string(), location());

    // Act
    let result = engine.finish();

    // Assert
    assert_eq!(result.unwrap_err().len(), 1);
}

#[test]
fn finish_succeeds_with_only_warnings() {
    // Arrange
    let mut engine = DiagnosticEngine::new();
    engine.emit_warning(ErrorCode::InvalidExpression, "odd".to_string(), location());

    // Act & Assert
    assert!(engine.finish().is_ok());
}

#[test]
fn max_errors_stops_collection() {
    // Arrange
    let mut engine = DiagnosticEngine::new();
    engine.set_max_errors(2);

    // Act
    engine.emit_error(
        ErrorCode::UndefinedVariable,
        "first".to_string(),
        location(),
    );
    engine.emit_error(
        ErrorCode::UndefinedVariable,
        "second".to_string(),
        location(),
    );

    // Assert
    let diagnostics = engine.into_errors();
    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].message, "first");
    assert_eq!(
        diagnostics[1].message,
        "Too many errors (2), stopping compilation"
    );
    assert_eq!(diagnostics[1].error_code, ErrorCode::GenericCompileError);
}

#[test]
fn take_diagnostics_resets_state() {
    // Arrange
    let mut engine = DiagnosticEngine::new();
    engine.emit_error(ErrorCode::UndefinedVariable, "bad".to_string(), location());
    engine.emit_warning(ErrorCode::InvalidExpression, "odd".to_string(), location());

    // Act
    let diagnostics = engine.take_diagnostics();

    // Assert
    assert_eq!(diagnostics.len(), 2);
    assert!(!engine.has_errors());
    assert_eq!(engine.error_count(), 0);
    assert_eq!(engine.warning_count(), 0);
    assert!(engine.take_diagnostics().is_empty());
}

#[test]
fn configuration_setters() {
    // Arrange
    let source = "let x = 1;";
    let mut engine = DiagnosticEngine::new();

    // Act
    engine.set_recovery_mode(true);
    engine.set_file_name("test.sl".to_string());
    engine.set_source_text(source);

    // Assert
    assert!(engine.is_recovery_mode());
}

#[test]
fn report_all_does_not_panic() {
    // Arrange
    let source = "let x = 1;\nlet y = ;";
    let mut engine = DiagnosticEngine::new();
    engine.emit_error(ErrorCode::UndefinedVariable, "bad".to_string(), location());
    engine.emit_warning(ErrorCode::InvalidExpression, "odd".to_string(), location());
    engine.emit_warning(ErrorCode::InvalidExpression, "odd".to_string(), location());
    engine.emit_with_suggestion(
        ErrorCode::ExpectedExpression,
        "missing".to_string(),
        Location::new(19, 2, 9, 1),
        Suggestion {
            message: "add an expression".to_string(),
            replacement: None,
            location: None,
        },
    );
    engine.emit(diagnostic(ErrorSeverity::Note));
    engine.emit_error(
        ErrorCode::UndefinedVariable,
        "out of range".to_string(),
        Location::new(0, 99, 1, 1),
    );

    // Act & Assert
    engine.report_all(source);
    DiagnosticEngine::new().report_all(source);
}
