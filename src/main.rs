#[derive(Debug, Clone, Copy, PartialEq)]
enum Tokentype {
    Identifier,     // x, y, myVar
    IntegerLiteral, // 123
    StringLiteral,  // "string"
    Let,            // let
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Invalid,        // Unrecognized token
    Equal,          // =
    Colon,          // :     
    TypeInt,        // int
    TypeString,     // string 
}

#[derive(Debug)]
struct Token {
    token_type: Tokentype,
    value: String,
}

impl Token {
    fn new(token_type: Tokentype, value: String) -> Token {
        Token { token_type, value }
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Skip whitespace
            c if c.is_whitespace() => {
                chars.next();
            }

            // Identifiers and keywords
            c if c.is_alphabetic() => {
                let mut identifier = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Check for keywords
                if identifier == "let" {
                    tokens.push(Token::new(Tokentype::Let, identifier));
                } else if identifier == "int" {
                    tokens.push(Token::new(Tokentype::TypeInt, identifier));
                } else if identifier == "string" {
                    tokens.push(Token::new(Tokentype::TypeString, identifier));
                } else {
                    tokens.push(Token::new(Tokentype::Identifier, identifier));
                }
            }

            // Integer literals
            c if c.is_digit(10) => {
                let mut number = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::new(Tokentype::IntegerLiteral, number));
            }

            // String literals
            '"' => {
                chars.next(); // Skip opening quote
                let mut string = String::new();

                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next(); // Skip closing quote
                        break;
                    } else {
                        string.push(c);
                        chars.next();
                    }
                }

                tokens.push(Token::new(Tokentype::StringLiteral, string));
            }
            ':' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Colon, ":".to_string()));
            }
            // Operators
            '+' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Plus, "+".to_string()));
            }
            '-' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Minus, "-".to_string()));
            }
            '*' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Multiply, "*".to_string()));
            }
            '/' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Divide, "/".to_string()));
            }
            '=' => {
                chars.next();
                tokens.push(Token::new(Tokentype::Equal, "=".to_string()));
            }

            // Invalid characters
            _ => {
                let invalid_char = chars.next().unwrap();
                tokens.push(Token::new(Tokentype::Invalid, invalid_char.to_string()));
            }
        }
    }
    tokens
}

// AST Node types
#[derive(Debug)]
enum Expression {
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Variable(String),
}

#[derive(Debug)]
enum Statement {
    Let(LetStatement),
    Expression(Expression),
}

#[derive(Debug)]
struct LiteralExpr {
    value: Value,
    expr_type: Type, // Track the expression's type
}
#[derive(Debug, Clone, PartialEq)]
enum Type {
    Integer,
    String,
    Unknown, // Used during type inference
}

#[derive(Debug)]
enum Value {
    Integer(i64),
    String(String),
}

#[derive(Debug)]
struct BinaryExpr {
    left: Box<Expression>,
    operator: Tokentype,
    right: Box<Expression>,
    expr_type: Type, // Track the expression's type
}

#[derive(Debug)]
struct LetStatement {
    name: String,
    value: Expression,
    expr_type: Type, // Track the expression's type
}

struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_token(Tokentype::Let) {
            self.let_statement()
        } else {
            self.expression_statement()
        }
    }

    fn let_statement(&mut self) -> Result<Statement, String> {
        if !self.check(Tokentype::Identifier) {
            return Err("Expected identifier after 'let'".to_string());
        }

        let name = self.advance().value.clone();
        let mut var_type = Type::Unknown; // Default to unknown, will be inferred

        // Check for type annotation (let x: int = ...)
        if self.match_token(Tokentype::Colon) {
            if self.match_token(Tokentype::TypeInt) {
                var_type = Type::Integer;
            } else if self.match_token(Tokentype::TypeString) {
                var_type = Type::String;
            } else {
                return Err("Expected type after colon".to_string());
            }
        }

        if !self.match_token(Tokentype::Equal) {
            return Err("Expected '=' after variable name".to_string());
        }

        let expr = self.expression()?;

        Ok(Statement::Let(LetStatement {
            name,
            value: expr,
            expr_type: var_type,
        }))
    }

    fn expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.expression()?;
        Ok(Statement::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expression, String> {
        self.term()
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut expr = self.factor()?;

        while self.match_any(&[Tokentype::Plus, Tokentype::Minus]) {
            let operator = self.previous().token_type.clone();
            let right = self.factor()?;
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: Type::Unknown, // Default to unknown, will be inferred
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.primary()?;

        while self.match_any(&[Tokentype::Multiply, Tokentype::Divide]) {
            let operator = self.previous().token_type.clone();
            let right = self.primary()?;
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                expr_type: Type::Unknown, // Default to unknown, will be inferred
            });
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_token(Tokentype::IntegerLiteral) {
            let value_str = self.previous().value.clone();
            let value = value_str
                .parse::<i64>()
                .map_err(|_| format!("Invalid integer: {}", value_str))?;
            return Ok(Expression::Literal(LiteralExpr {
                value: Value::Integer(value),
                expr_type: Type::Integer,
            }));
        }

        if self.match_token(Tokentype::StringLiteral) {
            let value = self.previous().value.clone();
            return Ok(Expression::Literal(LiteralExpr {
                value: Value::String(value),
                expr_type: Type::String,
            }));
        }

        if self.match_token(Tokentype::Identifier) {
            return Ok(Expression::Variable(self.previous().value.clone()));
        }

        Err(format!("Expected expression, found {:?}", self.peek()))
    }

    fn match_token(&mut self, token_type: Tokentype) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_any(&mut self, types: &[Tokentype]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: Tokentype) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

fn parse(tokens: &[Token]) -> Result<Vec<Statement>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

struct TypeChecker {
    // Could store a symbol table for variable types
    variables: std::collections::HashMap<String, Type>,
}

impl TypeChecker {
    fn new() -> Self {
        TypeChecker {
            variables: std::collections::HashMap::new(),
        }
    }

    fn check(&mut self, statements: &mut [Statement]) -> Result<(), String> {
        for stmt in statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &mut Statement) -> Result<(), String> {
        match stmt {
            Statement::Let(let_stmt) => {
                // Check and infer type of the expression
                let expr_type = self.check_expression(&mut let_stmt.value)?;

                // If type wasn't specified, infer it
                if let_stmt.expr_type == Type::Unknown {
                    let_stmt.expr_type = expr_type.clone();
                } else if let_stmt.expr_type != expr_type {
                    // Type mismatch
                    return Err(format!(
                        "Type mismatch: variable {} is {:?} but expression is {:?}",
                        let_stmt.name, let_stmt.expr_type, expr_type
                    ));
                }

                // Add to symbol table
                self.variables
                    .insert(let_stmt.name.clone(), let_stmt.expr_type.clone());
            }
            Statement::Expression(expr) => {
                self.check_expression(expr)?;
            }
        }
        Ok(())
    }

    fn check_expression(&mut self, expr: &mut Expression) -> Result<Type, String> {
        match expr {
            Expression::Literal(lit_expr) => {
                // Infer type from literal
                let inferred = match lit_expr.value {
                    Value::Integer(_) => Type::Integer,
                    Value::String(_) => Type::String,
                };
                lit_expr.expr_type = inferred.clone();
                Ok(inferred)
            }
            Expression::Binary(bin_expr) => {
                // Check operand types
                let left_type = self.check_expression(&mut bin_expr.left)?;
                let right_type = self.check_expression(&mut bin_expr.right)?;

                // Type checking rules for binary operations
                match (bin_expr.operator, &left_type, &right_type) {
                    // Integer arithmetic
                    (Tokentype::Plus, Type::Integer, Type::Integer) => {
                        bin_expr.expr_type = Type::Integer;
                    }
                    (Tokentype::Minus, Type::Integer, Type::Integer) => {
                        bin_expr.expr_type = Type::Integer;
                    }
                    (Tokentype::Multiply, Type::Integer, Type::Integer) => {
                        bin_expr.expr_type = Type::Integer;
                    }
                    (Tokentype::Divide, Type::Integer, Type::Integer) => {
                        bin_expr.expr_type = Type::Integer;
                    }

                    // String concatenation
                    (Tokentype::Plus, Type::String, Type::String) => {
                        bin_expr.expr_type = Type::String;
                    }

                    // Type error
                    _ => {
                        return Err(format!(
                            "Invalid operation: {:?} {:?} {:?}",
                            left_type, bin_expr.operator, right_type
                        ));
                    }
                }

                Ok(bin_expr.expr_type.clone())
            }
            Expression::Variable(name) => {
                // Look up variable type
                if let Some(var_type) = self.variables.get(name) {
                    Ok(var_type.clone())
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
        }
    }
}

fn print_ast(statements: &[Statement]) {
    println!("AST Root");
    for (i, stmt) in statements.iter().enumerate() {
        print_statement(stmt, 1, i == statements.len() - 1);
    }
}

fn print_statement(stmt: &Statement, indent: usize, is_last: bool) {
    let prefix = if is_last { "└── " } else { "├── " };

    match stmt {
        Statement::Let(let_stmt) => {
            println!(
                "{}{}{} = ",
                " ".repeat(indent * 4 - 4),
                prefix,
                let_stmt.name
            );
            print_expression(&let_stmt.value, indent + 1, true);
        }
        Statement::Expression(expr) => {
            println!("{}{}Expression", " ".repeat(indent * 4 - 4), prefix);
            print_expression(expr, indent + 1, true);
        }
    }
}

fn print_expression(expr: &Expression, indent: usize, is_last: bool) {
    let prefix = if is_last { "└── " } else { "├── " };

    match expr {
        Expression::Literal(lit_expr) => match &lit_expr.value {
            Value::Integer(i) => println!("{}{}Int: {}", " ".repeat(indent * 4 - 4), prefix, i),
            Value::String(s) => {
                println!("{}{}String: \"{}\"", " ".repeat(indent * 4 - 4), prefix, s)
            }
        },
        Expression::Binary(bin_expr) => {
            let op_str = match bin_expr.operator {
                Tokentype::Plus => "+",
                Tokentype::Minus => "-",
                Tokentype::Multiply => "*",
                Tokentype::Divide => "/",
                _ => "?",
            };

            println!("{}{}Op: {}", " ".repeat(indent * 4 - 4), prefix, op_str);
            print_expression(&bin_expr.left, indent + 1, false);
            print_expression(&bin_expr.right, indent + 1, true);
        }
        Expression::Variable(name) => {
            println!("{}{}Var: {}", " ".repeat(indent * 4 - 4), prefix, name);
        }
    }
}

fn repl() {
    let mut type_checker = TypeChecker::new();
    loop {
        let mut input = String::new();
        println!("> ");
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" || input.trim() == "" {
            break;
        }

        let tokens = tokenize(&input);
        for token in &tokens {
            println!("{:?}", token);
        }

        match parse(&tokens) {
            Ok(mut ast) => {
                // Type check the AST
                match type_checker.check(&mut ast) {
                    Ok(_) => {
                        println!("Type checking passed!");
                        print_ast(&ast);
                    }
                    Err(e) => {
                        println!("Type error: {}", e);
                    }
                }
            }
            Err(e) => println!("Parse error: {}", e),
        }
    }
}

fn main() {
    repl();
}
