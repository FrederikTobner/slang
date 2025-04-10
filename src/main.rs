mod token;
mod lexer;
mod type_checker;
mod ast;
mod parser;
mod visitor;
mod ast_printer;

use token::Token;
use lexer::tokenize;
use type_checker::TypeChecker;
use parser::Parser;
use ast::Statement;
use ast_printer::ASTPrinter;

fn parse(tokens: &[Token]) -> Result<Vec<Statement>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
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
            Ok(ast) => {
                // Type check the AST
                match type_checker.check(& ast) {
                    Ok(_) => {
                        println!("Type checking passed!");
                        let mut printer = ASTPrinter::new();
                        printer.print(&ast);                    }
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
