use slang_error::{DomainError, ErrorCategory, ErrorCode, Location, TypeError};

fn location() -> Location {
    Location::new_simple(7, 2, 3)
}

#[test]
fn type_not_found() {
    let build = |suggestions: Vec<String>| TypeError::TypeNotFound {
        name: "Foo".to_string(),
        suggestions,
        location: location(),
    };

    let without = build(vec![]);
    let compiler_error = without.to_compiler_error();
    assert_eq!(compiler_error.error_code, ErrorCode::UndefinedType);
    assert_eq!(compiler_error.message, "Type not found: Foo");
    assert_eq!(compiler_error.line, 2);
    assert_eq!(compiler_error.column, 3);
    assert_eq!(without.short_description(), "Type not found: Foo");
    assert_eq!(without.to_string(), "Type not found: Foo");

    let with = build(vec!["Bar".to_string(), "Baz".to_string()]);
    assert_eq!(
        with.to_compiler_error().message,
        "Type not found: Foo. Did you mean: Bar, Baz?"
    );
}

#[test]
fn circular_definition() {
    let error = TypeError::CircularDefinition {
        name: "A".to_string(),
        dependency_chain: vec!["A".to_string(), "B".to_string(), "A".to_string()],
        location: location(),
    };
    let compiler_error = error.to_compiler_error();

    assert_eq!(compiler_error.error_code, ErrorCode::CircularDependency);
    assert_eq!(
        compiler_error.message,
        "Circular type definition for 'A': A -> B -> A"
    );
    assert_eq!(error.short_description(), "Circular definition: A");
}

#[test]
fn invalid_type_construction() {
    let error = TypeError::InvalidTypeConstruction {
        attempted_type: "[i32; -1]".to_string(),
        reason: "negative size".to_string(),
        location: location(),
    };
    let compiler_error = error.to_compiler_error();

    assert_eq!(compiler_error.error_code, ErrorCode::InvalidType);
    assert_eq!(
        compiler_error.message,
        "Invalid type construction for '[i32; -1]': negative size"
    );
    assert_eq!(error.short_description(), "Invalid type: [i32; -1]");
    assert_eq!(error.location().position, 7);
}

#[test]
fn category_is_type() {
    let error = TypeError::TypeNotFound {
        name: "Foo".to_string(),
        suggestions: vec![],
        location: location(),
    };

    assert_eq!(error.category(), ErrorCategory::Type);
}
