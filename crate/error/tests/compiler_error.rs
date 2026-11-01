use slang_error::{CompilationError, ErrorCode, ErrorCollector, LineInfo};

fn error_at(position: usize, token_length: Option<usize>) -> CompilationError {
    CompilationError::new(
        ErrorCode::ExpectedSemicolon,
        "Missing semicolon".to_string(),
        1,
        1,
        position,
        token_length,
    )
}

#[test]
fn display() {
    let error = error_at(0, Some(1));

    assert_eq!(error.to_string(), "[E1001]: Missing semicolon");
}

#[test]
fn line_info_line_and_column() {
    let source = "let a = 1;\nlet b = 2;\nlet c = 3;";
    let line_info = LineInfo::new(source);

    assert_eq!(line_info.get_line_col(0), (1, 1));
    assert_eq!(line_info.get_line_col(4), (1, 5));
    assert_eq!(line_info.get_line_col(11), (2, 1));
    assert_eq!(line_info.get_line_col(25), (3, 4));
}

#[test]
fn line_info_line_text() {
    let line_info = LineInfo::new("first\nsecond\nthird");

    assert_eq!(line_info.get_line_text(1), Some("first"));
    assert_eq!(line_info.get_line_text(2), Some("second"));
    assert_eq!(line_info.get_line_text(3), Some("third"));
    assert_eq!(line_info.get_line_text(4), None);
}

#[test]
fn format_for_display_contains_context() {
    let source = "let a = 1\nlet b = 2;";
    let line_info = LineInfo::new(source);
    let output = error_at(9, Some(1)).format_for_display(&line_info);

    assert!(output.contains("E1001"));
    assert!(output.contains("Expected semicolon after statement"));
    assert!(output.contains("let a = 1"));
    assert!(output.contains("Missing semicolon"));
}

#[test]
fn format_for_display_without_token_length() {
    let line_info = LineInfo::new("let a = 1");
    let output = error_at(0, None).format_for_display(&line_info);

    assert!(output.contains('^'));
}

#[test]
fn error_collector_collects_and_takes_errors() {
    let mut collector = ErrorCollector::default();
    assert!(!collector.has_errors());

    collector.add_error(error_at(0, Some(1)));
    assert!(collector.has_errors());

    let errors = collector.take_errors();
    assert_eq!(errors.len(), 1);
    assert!(!collector.has_errors());
}

#[test]
fn error_collector_clear() {
    let mut collector = ErrorCollector::new();
    collector.add_error(error_at(0, Some(1)));
    collector.clear();

    assert!(!collector.has_errors());
}

#[test]
fn report_errors_does_not_panic() {
    let errors = vec![error_at(0, Some(3))];
    slang_error::report_errors(&errors, "let a = 1");

    let mut collector = ErrorCollector::new();
    collector.add_error(error_at(0, Some(3)));
    collector.report_errors();
}
