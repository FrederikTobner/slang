//! Test to understand HList execution order

use slang_compilation_pipeline::{
    error::StageError,
    hlist::{HCons, HNil, Execute},
    stage::{CompilationStage, StageContext},
};
use slang_shared::DiagnosticEngine;

struct DebugStage {
    name: &'static str,
}

impl CompilationStage for DebugStage {
    type Input = String;
    type Output = String;

    fn execute(&self, input: String, _context: &mut StageContext, _diagnostics: &mut DiagnosticEngine) -> Result<String, StageError> {
        println!("DebugStage '{}' executing with input: {}", self.name, input.len());
        Ok(format!("{}->{}", input, self.name))
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn is_critical(&self) -> bool {
        true
    }
}

#[test]
fn hlist_execution_order() {
    let stage1 = DebugStage { name: "Stage1" };
    let stage2 = DebugStage { name: "Stage2" };
    let stage3 = DebugStage { name: "Stage3" };
    
    // Create HList: Stage3 -> Stage2 -> Stage1 -> HNil
    let hlist = HCons::new(stage3, HCons::new(stage2, HCons::new(stage1, HNil)));
    
    let mut context = StageContext::new("input".to_string(), None);
    let mut diagnostics = DiagnosticEngine::new();
    
    let result = hlist.execute("input".to_string(), &mut context, &mut diagnostics);
    println!("Result: {result:?}");
    
    // This should show us the execution order
}
