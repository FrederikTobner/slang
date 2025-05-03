use crate::ast::{BinaryExpr, Expression, FunctionCallExpr, FunctionDeclarationStmt, LetStatement, LiteralExpr, Statement, TypeDefinitionStmt, UnaryExpr, LiteralValue};
use crate::token::Tokentype;
use crate::visitor::Visitor;

/// A visitor implementation that prints the AST in a human-readable format
pub struct ASTPrinter {
    /// Current indentation level for tree-like output
    indent_level: usize,
}

impl ASTPrinter {
    /// Creates a new AST printer with no indentation
    #[allow(dead_code)]
    pub fn new() -> Self {
        ASTPrinter { indent_level: 0 }
    }

    /// Prints the AST for a list of statements
    /// 
    /// ### Arguments
    /// 
    /// * `statements` - The statements to print
    #[allow(dead_code)]
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

    fn visit_type_definition_statement(&mut self, stmt: &TypeDefinitionStmt) -> () {
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
            Expression::Variable(name) => self.visit_variable_expression(name),
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
            LiteralValue::I32(i) => println!("{}I32: {}", self.indent(), i),
            LiteralValue::I64(i) => println!("{}I64: {}", self.indent(), i),
            LiteralValue::U32(u) => println!("{}U32: {}", self.indent(), u),
            LiteralValue::U64(u) => println!("{}U64: {}", self.indent(), u),
            LiteralValue::UnspecifiedInteger(i) => println!("{}UnspecifiedInteger: {}", self.indent(), i),
            LiteralValue::F64(f) => println!("{}F64: {}", self.indent(), f),
            LiteralValue::F32(f) => println!("{}F32: {}", self.indent(), f),
            LiteralValue::UnspecifiedFloat(f) => println!("{}UnspecifiedFloat: {}", self.indent(), f),
            LiteralValue::Boolean(b) => println!("{}Boolean: {}", self.indent(), b),
            LiteralValue::String(s) => println!("{}String: \"{}\"", self.indent(), s),
        }
    }

    fn visit_binary_expression(&mut self, bin_expr: &BinaryExpr) {
        let op_str = match bin_expr.operator {
            Tokentype::Plus => "+",
            Tokentype::Minus => "-",
            Tokentype::Multiply => "*",
            Tokentype::Divide => "/",
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
            Tokentype::Minus => "-",
            _ => "?",
        };

        println!("{}Unary: {}", self.indent(), op_str);

        self.indent_level += 1;
        self.visit_expression(&unary_expr.right);
        self.indent_level -= 1;
    }

    fn visit_variable_expression(&mut self, name: &str) {
        println!("{}Var: {}", self.indent(), name);
    }
}
