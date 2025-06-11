mod cli;
mod error;
mod exit;
mod compilation_pipeline;
mod compiler;

use clap::Parser;

/// Application entry point
fn main() {
    if let Err(err) = run() {
        exit::with_code(err.exit_code(), &err.to_string());
    }
}

/// Main application logic separated from exit handling for testability
fn run() -> error::CliResult<()> {
    let input = cli::Parser::parse();
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true);

    match &input.command {
        Some(cli::Commands::Compile { input, output }) => {
            cli::compile_file(input, output.clone())
        }

        Some(cli::Commands::Run { input }) => {
            cli::run_file(input)
        }

        Some(cli::Commands::Execute { input }) => {
            cli::execute_file(input)
        }
        
        None => {
            Err(error::CliError::Generic {
                message: "No command provided. Use --help for usage information.".to_string(),
                exit_code: exit::Code::Usage,
            })
        }
    }
}
