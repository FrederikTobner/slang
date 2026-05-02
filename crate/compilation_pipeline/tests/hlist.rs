use slang_compilation_pipeline::SlangSourceFile;
use slang_compilation_pipeline::hlist;
use slang_compilation_pipeline::{
    hlist::{Execute, HCons, HNil},
    stage::StageContext,
    stages::{ParsingStage, TokenizationStage},
};
use slang_shared::DiagnosticEngine;

#[test]
fn hlist_creation() {
    let _empty_list: HNil = HNil;
    let _single_item = HCons::new(TokenizationStage, HNil);
    let _multi_item = HCons::new(TokenizationStage, HCons::new(ParsingStage, HNil));
}

#[test]
fn hlist_macro() {
    let _list = hlist![TokenizationStage, ParsingStage];
}

#[test]
fn hlist_execution() {
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    let mut context = StageContext::new(
        source_file.content().to_string(),
        Some(source_file.file_name().to_string()),
    );
    let mut diagnostics = DiagnosticEngine::new();

    let list = hlist![TokenizationStage, ParsingStage];
    let result = list.execute(source_file, &mut context, &mut diagnostics);

    assert!(result.is_ok());
}

#[test]
fn hlist_single_stage() {
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    let mut context = StageContext::new(
        source_file.content().to_string(),
        Some(source_file.file_name().to_string()),
    );
    let mut diagnostics = DiagnosticEngine::new();

    let list = hlist![TokenizationStage];
    let result = list.execute(source_file, &mut context, &mut diagnostics);

    assert!(result.is_ok());
}

#[test]
fn hlist_empty() {
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    let mut context = StageContext::new(
        source_file.content().to_string(),
        Some(source_file.file_name().to_string()),
    );
    let mut diagnostics = DiagnosticEngine::new();

    let list: HNil = HNil;
    let result = list.execute(source_file.clone(), &mut context, &mut diagnostics);

    // Empty list should succeed with original input
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), source_file);
}

#[test]
fn hlist_type_safety() {
    // Different types can be stored in HList
    let _mixed_list = HCons::new(42, HCons::new("hello", HCons::new(true, HNil)));

    // Pipeline stages work too
    let _stage_list = HCons::new(TokenizationStage, HCons::new(ParsingStage, HNil));
}

#[test]
fn doctest_equivalent() {
    // Equivalent to the doctest example
    let source_file = SlangSourceFile::new("test.sl", "let x = 42;".to_string()).unwrap();
    let mut context = StageContext::new(
        source_file.content().to_string(),
        Some(source_file.file_name().to_string()),
    );
    let mut diagnostics = DiagnosticEngine::new();

    let stages = hlist![TokenizationStage, ParsingStage];
    let result = stages.execute(source_file, &mut context, &mut diagnostics);

    assert!(result.is_ok());
}
