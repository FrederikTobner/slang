mod cli;
mod exit;

/// Application entry point
fn main() {
    let input = cli::parse_args();
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true);
    match &input.command {
        Some(cli::Commands::Repl {}) => {
            cli::repl();
        }

        Some(cli::Commands::Compile { input, output }) => {
            cli::comp(input, output.clone());
        }

        Some(cli::Commands::Run { input }) => {
            cli::run(input);
        }

        Some(cli::Commands::Execute { input }) => {
            cli::execute(input);
        }

        None => {
            cli::repl();
        }
    }
}
