use slang_ir::ast::Statement;
use super::AstFormatter;

/// Compact formatter for single-line overview
pub struct CompactFormatter;

impl AstFormatter for CompactFormatter {
    fn format(&self, statements: &[Statement]) -> Result<String, Box<dyn std::error::Error>> {
        let mut printer = CompactAstPrinter::new();
        printer.print_statements(statements)
    }
}

/// Compact AST printer using simple approach
struct CompactAstPrinter;

impl CompactAstPrinter {
    fn new() -> Self {
        Self
    }

    fn print_statements(&mut self, statements: &[Statement]) -> Result<String, Box<dyn std::error::Error>> {
        let statement_types: Vec<String> = statements
            .iter()
            .map(get_statement_type_name)
            .collect();
        
        Ok(format!(
            "AST({} statements: [{}])", 
            statements.len(),
            statement_types.join(", ")
        ))
    }
}

/// Get simplified statement type name for compact format
fn get_statement_type_name(stmt: &Statement) -> String {
    match stmt {
        Statement::Let(_) => "Let".to_string(),
        Statement::Assignment(_) => "Assignment".to_string(),
        Statement::Return(_) => "Return".to_string(),
        Statement::Expression(_) => "Expression".to_string(),
        Statement::FunctionDeclaration(_) => "FunctionDecl".to_string(),
        Statement::TypeDefinition(_) => "TypeDef".to_string(),
        Statement::If(_) => "If".to_string(),
    }
}
