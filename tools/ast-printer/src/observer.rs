use slang_compilation_pipeline::observer::StageObserver;
use slang_ir::ast::Statement;
use slang_frontend::Token;
use colored::Colorize;

/// Observer that prints AST when parsing or semantic analysis completes
/// This observer is primarily used for debugging and development purposes
pub struct ASTPrinter {
}

impl ASTPrinter {
    pub fn new() -> Self {
        Self {}
    }

    fn print_ast(&self, statements: &[Statement]) {
        println!("{}", "=== AST ===".bright_green().bold());
        println!("Found {} statements", statements.len());
        for (i, stmt) in statements.iter().enumerate() {
            println!("Statement {i}: {stmt:?}");
        }
        println!("{}", "=== END AST ===".bright_green().bold());
    }
}

impl Default for ASTPrinter {
    fn default() -> Self {
        Self::new()
    }
}

// Implementation for parsing stage (tokens -> AST)
impl StageObserver<Vec<Token>, Vec<Statement>> for ASTPrinter {
    fn on_stage_success(&self, output: &Vec<Statement>) {
        self.print_ast(output);
    }
}

// Implementation for semantic analysis stage (AST -> AST)
impl StageObserver<Vec<Statement>, Vec<Statement>> for ASTPrinter {
    fn on_stage_success(&self, output: &Vec<Statement>) {
        self.print_ast(output);
    }
}
