use slang_compilation_pipeline::pipeline::{
    hlist::{HNil, HCons, Execute},
    stages::{TokenizationStage, ParsingStage},
    stage::StageContext,
};
use slang_compilation_pipeline::hlist;
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
    let source = "let x = 42;";
    let mut context = StageContext::new(source.to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    let list = hlist![TokenizationStage, ParsingStage];
    let result = list.execute(source.to_string(), &mut context, &mut diagnostics);
    
    assert!(result.is_ok());
}

#[test]
fn hlist_single_stage() {
    let source = "let x = 42;";
    let mut context = StageContext::new(source.to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    let list = hlist![TokenizationStage];
    let result = list.execute(source.to_string(), &mut context, &mut diagnostics);
    
    assert!(result.is_ok());
}

#[test]
fn hlist_empty() {
    let source = "let x = 42;";
    let mut context = StageContext::new(source.to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    let list: HNil = HNil;
    let result = list.execute(source, &mut context, &mut diagnostics);
    
    // Empty list should succeed with original input
    assert!(result.is_ok());
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
    let source = "let x = 42;";
    let mut context = StageContext::new(source.to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    let stages = hlist![TokenizationStage, ParsingStage];
    let result = stages.execute(source.to_string(), &mut context, &mut diagnostics);
    
    assert!(result.is_ok());
}
