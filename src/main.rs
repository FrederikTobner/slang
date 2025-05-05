mod ast;
mod ast_printer;
mod bytecode;
mod compiler;
mod init;
mod lexer;
mod parser;
mod token;
mod type_checker;
mod types;
mod value;
mod visitor;
mod vm;

use clap::{Parser as ClapParser, Subcommand};
use init::{comp, execute, repl, run};

/// Command line interface for the Slang language
#[derive(ClapParser)]
#[command(
    version,
    about = "Slang programming language",
    long_about = r#"Slang is a simple programming language designed for educational purposes.
It features a REPL, compilation to bytecode, and execution of both source files and compiled bytecode."#
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available commands for the Slang CLI
#[derive(Subcommand)]
enum Commands {
    /// Run the interactive REPL
    Repl {},

    /// Compile a Slang source file to bytecode
    Compile {
        /// Input source file
        input: String,

        /// Output bytecode file (default: same as input with .sip extension)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Run a compiled Slang bytecode file
    Run {
        /// Input compiled bytecode file
        input: String,
    },

    /// Run a Slang source file directly
    Execute {
        /// Input source file
        input: String,
    },
}

/// Application entry point
fn main() {
    let cli = Cli::parse();
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true);
    match &cli.command {
        Some(Commands::Repl {}) => {
            repl();
        }

        Some(Commands::Compile { input, output }) => {
            comp(input, output.clone());
        }

        Some(Commands::Run { input }) => {
            run(input);
        }

        Some(Commands::Execute { input }) => {
            execute(input);
        }

        None => {
            repl();
        }
    }
}
