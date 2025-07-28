use super::BenchmarkProgram;

/// Parser error case programs for testing error handling
pub const ERROR_MISSING_SEMICOLON: BenchmarkProgram =
    BenchmarkProgram::new("missing_semicolon", "let x = 1 + 2");

pub const ERROR_UNMATCHED_PAREN: BenchmarkProgram =
    BenchmarkProgram::new("unmatched_paren", "let x = (1 + 2;");

pub const ERROR_INVALID_SYNTAX: BenchmarkProgram =
    BenchmarkProgram::new("invalid_syntax", "let = x + 2;");

pub const ERROR_INCOMPLETE_FUNCTION: BenchmarkProgram =
    BenchmarkProgram::new("incomplete_function", "fn test() {");

/// Lexer error case programs for testing error handling
pub const ERROR_INVALID_CHAR: BenchmarkProgram =
    BenchmarkProgram::new("invalid_char", "let x = @#$%;");

pub const ERROR_UNTERMINATED_STRING: BenchmarkProgram =
    BenchmarkProgram::new("unterminated_string", r#"let x = "unterminated string"#);

pub const ERROR_INVALID_NUMBER: BenchmarkProgram =
    BenchmarkProgram::new("invalid_number", "let x = 123.456.789;");

pub const ERROR_MIXED_ERRORS: BenchmarkProgram =
    BenchmarkProgram::new("mixed_errors", r#"let @ = "unterminated #$%"#);

/// Semantic error case programs for testing error handling
pub const ERROR_UNDEFINED_VARIABLE: BenchmarkProgram =
    BenchmarkProgram::new("undefined_variable", "let x = undefined_var;");

pub const ERROR_TYPE_MISMATCH: BenchmarkProgram =
    BenchmarkProgram::new("type_mismatch", "let x: i32 = \"string\";");

pub const ERROR_UNDEFINED_FUNCTION: BenchmarkProgram =
    BenchmarkProgram::new("undefined_function", "let x = undefined_func();");

pub const ERROR_SCOPE_ERROR: BenchmarkProgram =
    BenchmarkProgram::new("scope_error", "{ let x = 1; } let y = x;");

pub const ERROR_RETURN_TYPE_MISMATCH: BenchmarkProgram = BenchmarkProgram::new(
    "return_type_mismatch",
    r#"
fn test() -> i32 {
    return "string";
}
"#,
);

pub const ERROR_PARAMETER_COUNT_MISMATCH: BenchmarkProgram = BenchmarkProgram::new(
    "parameter_count_mismatch",
    r#"
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
let result = add(1);
"#,
);
