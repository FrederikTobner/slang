mod cli;
mod format;
mod formatter;

use cli::{Parser, parse_and_print_ast};
use clap::Parser as ClapParser;
use colored::Colorize;
use std::process;

/// Application entry point
fn main() {
    if let Err(err) = run() {
        eprintln!("{}: {}", "Error".red().bold(), err);
        process::exit(1);
    }
}

/// Main application logic separated from exit handling
fn run() -> Result<(), String> {
    let args = Parser::parse();
    
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true);
    
    parse_and_print_ast(&args.input, args.format)
        .map_err(|e| e.to_string())
}
