mod ast;
mod ast_printer;
mod bytecode;
mod compiler;
mod lexer;
mod parser;
mod token;
mod type_checker;
mod visitor;
mod vm;

use ast::Statement;
use ast_printer::ASTPrinter;
use compiler::Compiler;
use lexer::tokenize;
use parser::Parser;
use token::Token;
use type_checker::TypeChecker;
use vm::VM;
use std::io::Write;

fn parse(tokens: &[Token]) -> Result<Vec<Statement>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

fn repl() {
    let mut type_checker = TypeChecker::new();
    let mut vm = VM::new();
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
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
                match type_checker.check(&ast) {
                    Ok(_) => {
                        println!("Type checking passed!");
                        let mut printer = ASTPrinter::new();
                        printer.print(&ast); // Compile the AST to bytecode
                        let mut compiler = Compiler::new();
                        match compiler.compile(&ast) {
                            Ok(chunk) => {
                                println!("\n=== Bytecode ===");
                                // Print bytecode for debugging
                                for (i, byte) in chunk.code.iter().enumerate() {
                                    print!("{:04} ", i);
                                    if let Some(op) = bytecode::OpCode::from_u8(*byte) {
                                        println!("{:?}", op);
                                    } else {
                                        println!("  {}", byte);
                                    }
                                }

                                // Execute the bytecode
                                println!("\n=== Execution ===");
                                if let Err(e) = vm.interpret(chunk) {
                                    println!("Runtime error: {}", e);
                                }
                            }
                            Err(e) => println!("Compilation error: {}", e),
                        }
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
