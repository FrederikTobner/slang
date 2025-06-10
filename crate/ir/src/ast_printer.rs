use crate::Visitor;
use crate::ast::{
    AssignmentStatement, BinaryExpr, BinaryOperator, BlockExpr, ConditionalExpr, Expression,
    FunctionCallExpr, FunctionDeclarationStmt, FunctionTypeExpr, IfStatement, LetStatement, LiteralExpr,
    LiteralValue, ReturnStatement, Statement, TypeDefinitionStmt, UnaryExpr, UnaryOperator,
    VariableExpr,
};
use slang_types::{
    TYPE_NAME_BOOL, TYPE_NAME_F32, TYPE_NAME_F64, TYPE_NAME_FLOAT, TYPE_NAME_I32, TYPE_NAME_I64,
    TYPE_NAME_INT, TYPE_NAME_STRING, TYPE_NAME_U32, TYPE_NAME_U64, TYPE_NAME_UNIT,
};

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
    fn visit_function_declaration_statement(&mut self, fn_decl: &FunctionDeclarationStmt) {
        println!(
            "{}Function: {} -> {:?}",
            self.indent(),
            fn_decl.name,
            fn_decl.return_type
        );

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
        self.visit_block_expression(&fn_decl.body);
        self.indent_level -= 2;
    }

    fn visit_return_statement(&mut self, return_stmt: &ReturnStatement) {
        println!("{}Return:", self.indent());
        if let Some(expr) = &return_stmt.value {
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

    fn visit_assignment_statement(&mut self, assign_stmt: &AssignmentStatement) {
        println!("{}Assignment: {} =", self.indent(), assign_stmt.name);
        self.indent_level += 1;
        assign_stmt.value.accept(self);
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
            LiteralValue::I64(i) => println!("{}{}: {}", self.indent(), TYPE_NAME_I64, i),
            LiteralValue::U32(u) => println!("{}{}: {}", self.indent(), TYPE_NAME_U32, u),
            LiteralValue::U64(u) => println!("{}{}: {}", self.indent(), TYPE_NAME_U64, u),
            LiteralValue::UnspecifiedInteger(i) => {
                println!("{}{}: {}", self.indent(), TYPE_NAME_INT, i)
            }
            LiteralValue::F64(f) => println!("{}{}: {}", self.indent(), TYPE_NAME_F64, f),
            LiteralValue::F32(f) => println!("{}{}: {}", self.indent(), TYPE_NAME_F32, f),
            LiteralValue::UnspecifiedFloat(f) => {
                println!("{}{}: {}", self.indent(), TYPE_NAME_FLOAT, f)
            }
            LiteralValue::Boolean(b) => println!("{}{}: {}", self.indent(), TYPE_NAME_BOOL, b),
            LiteralValue::String(s) => println!("{}{}: \"{}\"", self.indent(), TYPE_NAME_STRING, s),
            LiteralValue::Unit => println!("{}{}: ()", self.indent(), TYPE_NAME_UNIT),
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

    fn visit_variable_expression(&mut self, var_expr: &VariableExpr) {
        println!("{}Var: {}", self.indent(), var_expr.name);
    }

    fn visit_if_statement(&mut self, if_stmt: &IfStatement) {
        println!("{}If Statement:", self.indent());

        self.indent_level += 1;
        println!("{}Condition:", self.indent());
        self.indent_level += 1;
        self.visit_expression(&if_stmt.condition);
        self.indent_level -= 1;

        println!("{}Then Branch:", self.indent());
        self.indent_level += 1;
        self.visit_block_expression(&if_stmt.then_branch);
        self.indent_level -= 1;

        if let Some(else_branch) = &if_stmt.else_branch {
            println!("{}Else Branch:", self.indent());
            self.indent_level += 1;
            self.visit_block_expression(else_branch);
            self.indent_level -= 1;
        }

        self.indent_level -= 1;
    }

    fn visit_conditional_expression(&mut self, cond_expr: &ConditionalExpr) {
        println!("{}Conditional Expression:", self.indent());

        self.indent_level += 1;
        println!("{}Condition:", self.indent());
        self.indent_level += 1;
        self.visit_expression(&cond_expr.condition);
        self.indent_level -= 1;

        println!("{}Then:", self.indent());
        self.indent_level += 1;
        self.visit_expression(&cond_expr.then_branch);
        self.indent_level -= 1;

        println!("{}Else:", self.indent());
        self.indent_level += 1;
        self.visit_expression(&cond_expr.else_branch);
        self.indent_level -= 1;

        self.indent_level -= 1;
    }

    fn visit_function_type_expression(&mut self, func_type_expr: &FunctionTypeExpr) {
        print!("{}fn(", self.indent());
        for (i, _param_type) in func_type_expr.param_types.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("param_{}", i);
        }
        println!(") -> return_type");
    }

    fn visit_block_expression(&mut self, block_expr: &BlockExpr) {
        println!("{}Block Expression:", self.indent());

        self.indent_level += 1;

        if !block_expr.statements.is_empty() {
            println!("{}Statements:", self.indent());
            self.indent_level += 1;
            for stmt in &block_expr.statements {
                self.visit_statement(stmt);
            }
            self.indent_level -= 1;
        }

        if let Some(return_expr) = &block_expr.return_expr {
            println!("{}Return Expression:", self.indent());
            self.indent_level += 1;
            self.visit_expression(return_expr);
            self.indent_level -= 1;
        }

        self.indent_level -= 1;
    }
}
