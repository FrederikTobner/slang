use slang_compilation_pipeline::pipeline::observers::StageObserver;
use slang_ir::ast::Statement;
use slang_frontend::Token;
use colored::Colorize;

/// Observer that prints AST when parsing or semantic analysis completes
/// This observer is primarily used for debugging and development purposes
pub struct ASTPrintObserver {
    enabled: bool,
}

impl ASTPrintObserver {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn disabled() -> Self {
        Self { enabled: false }
    }

    fn print_ast(&self, statements: &Vec<Statement>) {
        if !self.enabled {
            return;
        }

        println!("{}", "=== AST ===".bright_green().bold());
        println!("Found {} statements", statements.len());
        for (i, stmt) in statements.iter().enumerate() {
            println!("Statement {}: {:?}", i, stmt);
        }
        println!("{}", "=== END AST ===".bright_green().bold());
    }
}

impl Default for ASTPrintObserver {
    fn default() -> Self {
        Self::new()
    }
}

// Implementation for parsing stage (tokens -> AST)
impl StageObserver<Vec<Token>, Vec<Statement>> for ASTPrintObserver {
    fn on_stage_success(&self, output: &Vec<Statement>) {
        self.print_ast(output);
    }
}

// Implementation for semantic analysis stage (AST -> AST)
impl StageObserver<Vec<Statement>, Vec<Statement>> for ASTPrintObserver {
    fn on_stage_success(&self, output: &Vec<Statement>) {
        self.print_ast(output);
    }
}
