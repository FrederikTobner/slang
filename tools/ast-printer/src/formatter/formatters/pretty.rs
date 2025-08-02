use slang_ir::ast::*;
use slang_ir::Visitor;
use colored::Colorize;
use slang_error::DomainResult;
use std::fmt::Write;
use super::{AstFormatter, FormatError};

/// Pretty formatter with colors and hierarchical structure
pub struct PrettyFormatter;

impl AstFormatter for PrettyFormatter {
    fn format(&self, statements: &[Statement]) -> Result<String, Box<dyn std::error::Error>> {
        let mut printer = PrettyAstPrinter::new();
        printer.print_statements(statements)
    }
}

/// Pretty AST printer using the visitor pattern
struct PrettyAstPrinter {
    output: String,
    indent_level: usize,
}

impl PrettyAstPrinter {
    fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
        }
    }

    fn print_statements(&mut self, statements: &[Statement]) -> Result<String, Box<dyn std::error::Error>> {
        writeln!(self.output, "{}({} statements)", "AST".blue().bold(), statements.len())?;
        
        for (i, statement) in statements.iter().enumerate() {
            self.indent_level = 1;
            writeln!(self.output, "{}{}:", format!("Statement {}", i).green().bold(), self.indent())?;
            self.indent_level = 2;
            statement.accept(self).map_err(|e| Box::new(FormatError::new(format!("Visitor error: {}", e))))?;
        }
        
        Ok(self.output.clone())
    }

    fn indent(&self) -> String {
        "  ".repeat(self.indent_level)
    }

    fn writeln_indented(&mut self, text: &str) -> Result<(), FormatError> {
        writeln!(self.output, "{}{}", self.indent(), text)
            .map_err(|e| FormatError::new(e.to_string()))
    }
}

impl Visitor<()> for PrettyAstPrinter {
    fn visit_let_statement(&mut self, stmt: &LetStatement) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}{}(name: {}, mutable: {}, type: {:?})",
            "Let".yellow().bold(),
            "Statement".yellow(),
            stmt.name.green(),
            stmt.is_mutable,
            stmt.expr_type
        ))?;
        
        self.indent_level += 1;
        self.writeln_indented("Value:")?;
        self.indent_level += 1;
        stmt.value.accept(self)?;
        self.indent_level -= 2;
        Ok(())
    }

    fn visit_assignment_statement(&mut self, stmt: &AssignmentStatement) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}{}(variable: {})",
            "Assignment".yellow().bold(),
            "Statement".yellow(),
            stmt.name.green()
        ))?;
        
        self.indent_level += 1;
        self.writeln_indented("Value:")?;
        self.indent_level += 1;
        stmt.value.accept(self)?;
        self.indent_level -= 2;
        Ok(())
    }

    fn visit_expression_statement(&mut self, expr: &Expression) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}{}",
            "Expression".yellow().bold(),
            "Statement".yellow()
        ))?;
        
        self.indent_level += 1;
        expr.accept(self)?;
        self.indent_level -= 1;
        Ok(())
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStatement) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}{}",
            "Return".yellow().bold(),
            "Statement".yellow()
        ))?;
        
        if let Some(ref expr) = stmt.value {
            self.indent_level += 1;
            self.writeln_indented("Value:")?;
            self.indent_level += 1;
            expr.accept(self)?;
            self.indent_level -= 2;
        }
        Ok(())
    }

    fn visit_function_declaration_statement(&mut self, stmt: &FunctionDeclarationStmt) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}{}(name: {}, params: {}, return_type: {:?})",
            "FunctionDeclaration".yellow().bold(),
            "Statement".yellow(),
            stmt.name.green(),
            stmt.parameters.len(),
            stmt.return_type
        ))?;
        
        self.indent_level += 1;
        if !stmt.parameters.is_empty() {
            self.writeln_indented("Parameters:")?;
            self.indent_level += 1;
            for param in &stmt.parameters {
                self.writeln_indented(&format!("{}: {:?}", param.name.green(), param.param_type))?;
            }
            self.indent_level -= 1;
        }
        
        self.writeln_indented("Body:")?;
        self.indent_level += 1;
        self.visit_block_expression(&stmt.body)?;
        self.indent_level -= 2;
        Ok(())
    }

    fn visit_type_definition_statement(&mut self, stmt: &TypeDefinitionStmt) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}{}(name: {}, fields: {})",
            "TypeDefinition".yellow().bold(),
            "Statement".yellow(),
            stmt.name.green(),
            stmt.fields.len()
        ))?;
        
        if !stmt.fields.is_empty() {
            self.indent_level += 1;
            self.writeln_indented("Fields:")?;
            self.indent_level += 1;
            for field in &stmt.fields {
                self.writeln_indented(&format!("{}: {:?}", field.0.green(), field.1))?;
            }
            self.indent_level -= 2;
        }
        Ok(())
    }

    fn visit_if_statement(&mut self, stmt: &IfStatement) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}{}",
            "If".yellow().bold(),
            "Statement".yellow()
        ))?;
        
        self.indent_level += 1;
        self.writeln_indented("Condition:")?;
        self.indent_level += 1;
        stmt.condition.accept(self)?;
        self.indent_level -= 1;
        
        self.writeln_indented("Then:")?;
        self.indent_level += 1;
        self.visit_block_expression(&stmt.then_branch)?;
        self.indent_level -= 1;
        
        if let Some(ref else_branch) = stmt.else_branch {
            self.writeln_indented("Else:")?;
            self.indent_level += 1;
            self.visit_block_expression(else_branch)?;
            self.indent_level -= 1;
        }
        self.indent_level -= 1;
        Ok(())
    }

    fn visit_literal_expression(&mut self, expr: &LiteralExpr) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}(value: {})",
            "Literal".cyan().bold(),
            format_literal_value(&expr.value).magenta()
        ))?;
        Ok(())
    }

    fn visit_variable_expression(&mut self, expr: &VariableExpr) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}(name: {})",
            "Variable".cyan().bold(),
            expr.name.green()
        ))?;
        Ok(())
    }

    fn visit_binary_expression(&mut self, expr: &BinaryExpr) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}(operator: {})",
            "Binary".cyan().bold(),
            format!("{:?}", expr.operator).red()
        ))?;
        
        self.indent_level += 1;
        self.writeln_indented("Left:")?;
        self.indent_level += 1;
        expr.left.accept(self)?;
        self.indent_level -= 1;
        
        self.writeln_indented("Right:")?;
        self.indent_level += 1;
        expr.right.accept(self)?;
        self.indent_level -= 2;
        Ok(())
    }

    fn visit_unary_expression(&mut self, expr: &UnaryExpr) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}(operator: {})",
            "Unary".cyan().bold(),
            format!("{:?}", expr.operator).red()
        ))?;
        
        self.indent_level += 1;
        self.writeln_indented("Operand:")?;
        self.indent_level += 1;
        expr.right.accept(self)?;
        self.indent_level -= 2;
        Ok(())
    }

    fn visit_call_expression(&mut self, expr: &FunctionCallExpr) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}(function: {}, args: {})",
            "Call".cyan().bold(),
            expr.name.green(),
            expr.arguments.len()
        ))?;
        
        if !expr.arguments.is_empty() {
            self.indent_level += 1;
            self.writeln_indented("Arguments:")?;
            self.indent_level += 1;
            for (i, arg) in expr.arguments.iter().enumerate() {
                self.writeln_indented(&format!("Arg {}:", i))?;
                self.indent_level += 1;
                arg.accept(self)?;
                self.indent_level -= 1;
            }
            self.indent_level -= 2;
        }
        Ok(())
    }

    fn visit_conditional_expression(&mut self, expr: &ConditionalExpr) -> DomainResult<()> {
        self.writeln_indented(&format!("{}", "Conditional".cyan().bold()))?;
        
        self.indent_level += 1;
        self.writeln_indented("Condition:")?;
        self.indent_level += 1;
        expr.condition.accept(self)?;
        self.indent_level -= 1;
        
        self.writeln_indented("Then:")?;
        self.indent_level += 1;
        expr.then_branch.accept(self)?;
        self.indent_level -= 1;
        
        self.writeln_indented("Else:")?;
        self.indent_level += 1;
        expr.else_branch.accept(self)?;
        self.indent_level -= 2;
        Ok(())
    }

    fn visit_block_expression(&mut self, expr: &BlockExpr) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}(statements: {})",
            "Block".cyan().bold(),
            expr.statements.len()
        ))?;
        
        if !expr.statements.is_empty() {
            self.indent_level += 1;
            for stmt in &expr.statements {
                stmt.accept(self)?;
            }
            self.indent_level -= 1;
        }
        
        if let Some(ref ret_expr) = expr.return_expr {
            self.indent_level += 1;
            self.writeln_indented("Return:")?;
            self.indent_level += 1;
            ret_expr.accept(self)?;
            self.indent_level -= 2;
        }
        Ok(())
    }

    fn visit_function_type_expression(&mut self, expr: &FunctionTypeExpr) -> DomainResult<()> {
        self.writeln_indented(&format!(
            "{}(params: {}, return: {:?})",
            "FunctionType".cyan().bold(),
            expr.param_types.len(),
            expr.return_type
        ))?;
        Ok(())
    }
}

/// Helper function to format literal values for display
fn format_literal_value(literal: &LiteralValue) -> String {
    match literal {
        LiteralValue::I32(i) => format!("{}i32", i),
        LiteralValue::I64(i) => format!("{}i64", i),
        LiteralValue::U32(i) => format!("{}u32", i),
        LiteralValue::U64(i) => format!("{}u64", i),
        LiteralValue::UnspecifiedInteger(i) => i.to_string(),
        LiteralValue::F32(f) => format!("{}f32", f),
        LiteralValue::F64(f) => format!("{}f64", f),
        LiteralValue::UnspecifiedFloat(f) => f.to_string(),
        LiteralValue::String(s) => format!("\"{}\"", s),
        LiteralValue::Boolean(b) => b.to_string(),
        LiteralValue::Unit => "()".to_string(),
    }
}
