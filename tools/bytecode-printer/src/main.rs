mod cli;
mod format;
mod formatter;
mod observer;

use cli::{Parser, analyze_bytecode};
use clap::Parser as ClapParser;
use colored::Colorize;
use std::process;

/// Application entry point
fn main() {
    if let Err(err) = run() {
        eprintln!("{}: {err}", "Error".red().bold());
        process::exit(1);
    }
}

/// Main application logic separated from exit handling
fn run() -> Result<(), String> {
    let args = Parser::parse();
    
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true);
    
    analyze_bytecode(&args.input, args.format, args.verbose)
        .map_err(|e| e.to_string())
}
