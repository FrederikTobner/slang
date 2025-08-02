use slang_ir::ast::*;
use serde_json::{json, Value};
use super::AstFormatter;

/// JSON formatter for structured output
pub struct JsonFormatter;

impl AstFormatter for JsonFormatter {
    fn format(&self, statements: &[Statement]) -> Result<String, Box<dyn std::error::Error>> {
        let mut printer = JsonAstPrinter::new();
        printer.print_statements(statements)
    }
}

/// JSON AST printer - simplified version
struct JsonAstPrinter;

impl JsonAstPrinter {
    fn new() -> Self {
        Self
    }

    fn print_statements(&mut self, statements: &[Statement]) -> Result<String, Box<dyn std::error::Error>> {
        let statement_values: Vec<Value> = statements
            .iter()
            .map(|stmt| simple_statement_to_json(stmt))
            .collect();
        
        let json_ast = json!({
            "type": "AST",
            "statement_count": statements.len(),
            "statements": statement_values
        });
        
        Ok(serde_json::to_string_pretty(&json_ast)?)
    }
}

/// Simplified statement to JSON conversion
fn simple_statement_to_json(stmt: &Statement) -> Value {
    match stmt {
        Statement::Let(let_stmt) => json!({
            "type": "LetStatement",
            "name": let_stmt.name,
            "is_mutable": let_stmt.is_mutable,
            "value": simple_expression_to_json(&let_stmt.value)
        }),
        Statement::Assignment(assign) => json!({
            "type": "AssignmentStatement",
            "name": assign.name,
            "value": simple_expression_to_json(&assign.value)
        }),
        Statement::Return(ret) => json!({
            "type": "ReturnStatement",
            "value": ret.value.as_ref().map(simple_expression_to_json)
        }),
        Statement::Expression(expr) => json!({
            "type": "ExpressionStatement",
            "expression": simple_expression_to_json(expr)
        }),
        Statement::FunctionDeclaration(func) => json!({
            "type": "FunctionDeclarationStatement",
            "name": func.name,
            "parameters": func.parameters.len(),
            "return_type": format!("{:?}", func.return_type)
        }),
        Statement::TypeDefinition(typedef) => json!({
            "type": "TypeDefinitionStatement",
            "name": typedef.name,
            "fields": typedef.fields.len()
        }),
        Statement::If(if_stmt) => json!({
            "type": "IfStatement",
            "condition": simple_expression_to_json(&if_stmt.condition),
            "has_else": if_stmt.else_branch.is_some()
        }),
    }
}

/// Simplified expression to JSON conversion
fn simple_expression_to_json(expr: &Expression) -> Value {
    match expr {
        Expression::Literal(lit) => json!({
            "type": "Literal",
            "value": literal_value_to_json(&lit.value)
        }),
        Expression::Variable(var) => json!({
            "type": "Variable",
            "name": var.name
        }),
        Expression::Binary(bin) => json!({
            "type": "Binary",
            "operator": format!("{:?}", bin.operator)
        }),
        Expression::Unary(un) => json!({
            "type": "Unary",
            "operator": format!("{:?}", un.operator)
        }),
        Expression::Call(call) => json!({
            "type": "Call",
            "function": call.name,
            "arguments": call.arguments.len()
        }),
        Expression::Conditional(_cond) => json!({
            "type": "Conditional"
        }),
        Expression::Block(block) => json!({
            "type": "BlockExpression",
            "statements": block.statements.len()
        }),
        Expression::FunctionType(func_type) => json!({
            "type": "FunctionType",
            "parameters": func_type.param_types.len()
        }),
    }
}

/// Helper function to convert literal values to JSON
fn literal_value_to_json(literal: &LiteralValue) -> Value {
    match literal {
        LiteralValue::I32(i) => json!(i),
        LiteralValue::I64(i) => json!(i),
        LiteralValue::U32(i) => json!(i),
        LiteralValue::U64(i) => json!(i),
        LiteralValue::UnspecifiedInteger(i) => json!(i),
        LiteralValue::F32(f) => json!(f),
        LiteralValue::F64(f) => json!(f),
        LiteralValue::UnspecifiedFloat(f) => json!(f),
        LiteralValue::String(s) => json!(s),
        LiteralValue::Boolean(b) => json!(b),
        LiteralValue::Unit => json!(null),
    }
}
