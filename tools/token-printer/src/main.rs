mod cli;
mod format;
mod formatters;
mod observer;

use clap::Parser;
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
    let args = cli::Parser::parse();
    
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true);
    
    cli::tokenize_file(&args.input, args.format)
        .map_err(|e| e.to_string())
}