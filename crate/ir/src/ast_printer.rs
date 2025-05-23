use crate::ast::{BinaryExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt, LetStatement, LiteralExpr, Statement, TypeDefinitionStmt, UnaryExpr, LiteralValue, UnaryOperator, BinaryOperator};
use crate::visitor::Visitor;
use slang_types::types::{TYPE_NAME_BOOL, TYPE_NAME_I32, TYPE_NAME_I64, TYPE_NAME_F32, TYPE_NAME_F64, TYPE_NAME_STRING, TYPE_NAME_INT, TYPE_NAME_FLOAT, TYPE_NAME_U32, TYPE_NAME_U64};

/// A visitor implementation that prints the AST in a human-readable format
pub struct ASTPrinter {
    /// Current indentation level for tree-like output
    indent_level: usize,
}

impl ASTPrinter {
    /// Creates a new AST printer with no indentation
    pub fn new() -> Self {
        ASTPrinter { indent_level: 0 }
    }

    /// Prints the AST for a list of statements
    /// 
    /// ### Arguments
    /// 
    /// * `statements` - The statements to print
    pub fn print(&mut self, statements: &[Statement]) {
        println!("AST Root");
        for stmt in statements {
            self.indent_level = 1;
            stmt.accept::<()>(self);
        }
    }

    /// Helper function to get the current indentation string
    fn indent(&self) -> String {
        " ".repeat(self.indent_level * 4)
    }
}

impl Visitor<()> for ASTPrinter {
    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let(let_stmt) => self.visit_let_statement(let_stmt),
            Statement::Expression(expr) => self.visit_expression_statement(expr),
            Statement::TypeDefinition(type_stmt) => self.visit_type_definition_statement(type_stmt),
            Statement::FunctionDeclaration(fn_decl) => self.visit_function_declaration_statement(fn_decl),
            Statement::Block(stmts) => self.visit_block_statement(stmts),
            Statement::Return(expr) => self.visit_return_statement(expr),
        }
    }

    fn visit_function_declaration_statement(&mut self, fn_decl: &FunctionDeclarationStmt) {
        println!("{}Function: {} -> {:?}", self.indent(), fn_decl.name, fn_decl.return_type);
        
        self.indent_level += 1;
        
        if !fn_decl.parameters.is_empty() {
            println!("{}Parameters:", self.indent());
            self.indent_level += 1;
            for param in &fn_decl.parameters {
                println!("{}{}: {:?}", self.indent(), param.name, param.param_type);
            }
            self.indent_level -= 1;
        }
        
        println!("{}Body:", self.indent());
        self.indent_level += 1;
        for stmt in &fn_decl.body {
            self.visit_statement(stmt);
        }
        self.indent_level -= 2;
    }
    
    fn visit_block_statement(&mut self, stmts: &[Statement]) {
        println!("{}Block:", self.indent());
        self.indent_level += 1;
        for stmt in stmts {
            self.visit_statement(stmt);
        }
        self.indent_level -= 1;
    }
    
    fn visit_return_statement(&mut self, expr: &Option<Expression>) {
        println!("{}Return:", self.indent());
        if let Some(expr) = expr {
            self.indent_level += 1;
            self.visit_expression(expr);
            self.indent_level -= 1;
        }
    }

    fn visit_let_statement(&mut self, let_stmt: &LetStatement) {
        println!("{}Let: {} =", self.indent(), let_stmt.name);
        self.indent_level += 1;
        let_stmt.value.accept(self);
        self.indent_level -= 1;
    }

    fn visit_expression_statement(&mut self, expr: &Expression) {
        println!("{}Expression:", self.indent());
        self.indent_level += 1;
        self.visit_expression(expr);
        self.indent_level -= 1;
    }

    fn visit_type_definition_statement(&mut self, stmt: &TypeDefinitionStmt) {
        println!("{}Type Definition: {}", self.indent(), stmt.name);
        self.indent_level += 1;
        for field in &stmt.fields {
            println!("{}Field: {}", self.indent(), field.0);
        }
        self.indent_level -= 1;
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(lit) => self.visit_literal_expression(lit),
            Expression::Binary(bin) => self.visit_binary_expression(bin),
            Expression::Variable(name, location) => self.visit_variable_expression(name, location),
            Expression::Unary(unary) => self.visit_unary_expression(unary),
            Expression::Call(call) => self.visit_call_expression(call),
        }
    }
    
    fn visit_call_expression(&mut self, call_expr: &FunctionCallExpr) {
        println!("{}Call: {}", self.indent(), call_expr.name);
        
        if !call_expr.arguments.is_empty() {
            self.indent_level += 1;
            println!("{}Arguments:", self.indent());
            self.indent_level += 1;
            for arg in &call_expr.arguments {
                self.visit_expression(arg);
            }
            self.indent_level -= 2;
        }
    }

    fn visit_literal_expression(&mut self, lit_expr: &LiteralExpr) {
        match &lit_expr.value {
            LiteralValue::I32(i) => println!("{}{}: {}", self.indent(), TYPE_NAME_I32, i),
            LiteralValue::I64(i) => println!("{}{}: {}", self.indent(),  TYPE_NAME_I64, i),
            LiteralValue::U32(u) => println!("{}{}: {}", self.indent(),  TYPE_NAME_U32, u),
            LiteralValue::U64(u) => println!("{}{}: {}", self.indent(),  TYPE_NAME_U64, u),
            LiteralValue::UnspecifiedInteger(i) => println!("{}{}: {}", self.indent(), TYPE_NAME_INT, i),
            LiteralValue::F64(f) => println!("{}{}: {}", self.indent(),  TYPE_NAME_F64, f),
            LiteralValue::F32(f) => println!("{}{}: {}", self.indent(),  TYPE_NAME_F32, f),
            LiteralValue::UnspecifiedFloat(f) => println!("{}{}: {}", self.indent(), TYPE_NAME_FLOAT,f),
            LiteralValue::Boolean(b) => println!("{}{}: {}",  self.indent(),  TYPE_NAME_BOOL, b),
            LiteralValue::String(s) => println!("{}{}: \"{}\"", self.indent(),  TYPE_NAME_STRING, s),
        }
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) {
        let op_str = match bin_expr.operator {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            _ => "?",
        };

        println!("{}Op: {}", self.indent(), op_str);

        self.indent_level += 1;
        self.visit_expression(&bin_expr.left);
        self.visit_expression(&bin_expr.right);
        self.indent_level -= 1;
    }

    fn visit_unary_expression(&mut self, unary_expr: &UnaryExpr) {
        let op_str = match unary_expr.operator {
            UnaryOperator::Negate => "-",
            _ => "?",
        };

        println!("{}Unary: {}", self.indent(), op_str);

        self.indent_level += 1;
        self.visit_expression(&unary_expr.right);
        self.indent_level -= 1;
    }

    fn visit_variable_expression(&mut self, name: &str, _location: &crate::source_location::SourceLocation) {
        println!("{}Var: {}", self.indent(), name);
    }
}
