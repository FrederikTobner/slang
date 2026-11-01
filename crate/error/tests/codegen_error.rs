use slang_error::{CodegenError, DomainError, ErrorCategory, ErrorCode, Location, ResourceType};

fn location() -> Location {
    Location::new_simple(5, 1, 6)
}

#[test]
fn resource_type_as_str() {
    assert_eq!(ResourceType::Constants.as_str(), "constants");
    assert_eq!(ResourceType::LocalVariables.as_str(), "local variables");
    assert_eq!(ResourceType::Variables.as_str(), "variables");
    assert_eq!(ResourceType::JumpDistance.as_str(), "jump distance");
    assert_eq!(ResourceType::Other("labels".to_string()).as_str(), "labels");
    assert_eq!(ResourceType::JumpDistance.to_string(), "jump distance");
}

#[test]
fn stack_overflow() {
    let error = CodegenError::StackOverflow {
        current_depth: 300,
        max_depth: 256,
        location: location(),
    };
    let compiler_error = error.to_compiler_error();

    assert_eq!(compiler_error.error_code, ErrorCode::StackOverflow);
    assert_eq!(
        compiler_error.message,
        "Stack overflow during compilation: depth 300 exceeds maximum 256"
    );
    assert_eq!(error.short_description(), "Stack overflow");
    assert_eq!(error.to_string(), "Stack overflow");
}

#[test]
fn limit_exceeded() {
    let error = CodegenError::LimitExceeded {
        resource: ResourceType::Constants,
        current: 300,
        limit: 256,
        location: location(),
    };
    let compiler_error = error.to_compiler_error();

    assert_eq!(compiler_error.error_code, ErrorCode::TooManyConstants);
    assert_eq!(
        compiler_error.message,
        "Too many constants: 300 exceeds limit of 256"
    );
    assert_eq!(error.short_description(), "Limit exceeded: constants");
}

#[test]
fn unsupported_feature() {
    let build = |alternative: Option<&str>| CodegenError::UnsupportedFeature {
        feature: "closures".to_string(),
        reason: "not implemented".to_string(),
        alternative: alternative.map(str::to_string),
        location: location(),
    };

    let without = build(None);
    assert_eq!(
        without.to_compiler_error().error_code,
        ErrorCode::UnsupportedFeature
    );
    assert_eq!(
        without.to_compiler_error().message,
        "Unsupported feature 'closures': not implemented"
    );
    assert_eq!(without.short_description(), "Unsupported feature: closures");

    assert_eq!(
        build(Some("use functions")).to_compiler_error().message,
        "Unsupported feature 'closures': not implemented. Try: use functions"
    );
}

#[test]
fn internal_error() {
    let error = CodegenError::InternalError {
        message: "boom".to_string(),
        location: location(),
    };
    let compiler_error = error.to_compiler_error();

    assert_eq!(compiler_error.error_code, ErrorCode::InternalError);
    assert_eq!(compiler_error.message, "Internal compiler error: boom");
    assert_eq!(error.short_description(), "Internal error");
    assert_eq!(error.category(), ErrorCategory::Codegen);
    assert_eq!(error.location().line, 1);
}
