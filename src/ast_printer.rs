use crate::ast::{BinaryExpr, Expression, LetStatement, LiteralExpr, Statement, TypeDefinitionStmt, UnaryExpr, Value};
use crate::token::Tokentype;
use crate::visitor::Visitor;

pub struct ASTPrinter {
    indent_level: usize,
}

impl ASTPrinter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ASTPrinter { indent_level: 0 }
    }

    #[allow(dead_code)]
    pub fn print(&mut self, statements: &[Statement]) {
        println!("AST Root");
        for stmt in statements {
            self.indent_level = 1;
            stmt.accept::<()>(self);
        }
    }

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
        }
    }

    fn visit_literal_expression(&mut self, lit_expr: &LiteralExpr) {
        match &lit_expr.value {
            Value::I32(i) => println!("{}I32: {}", self.indent(), i),
            Value::I64(i) => println!("{}I64: {}", self.indent(), i),
            Value::U32(u) => println!("{}U32: {}", self.indent(), u),
            Value::U64(u) => println!("{}U64: {}", self.indent(), u),
            Value::UnspecifiedInteger(i) => println!("{}UnspecifiedInteger: {}", self.indent(), i),
            Value::F64(f) => println!("{}F64: {}", self.indent(), f),
            Value::String(s) => println!("{}String: \"{}\"", self.indent(), s),
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
