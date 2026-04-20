use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;
use please_cli::cli::{Cli, CliAction};

fn main() {
    if let Err(error) = run() {
        eprintln!("{} {error:#}", "error:".red().bold());
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.into_command_input() {
        CliAction::Init(shell) => {
            print!("{}", shell.init_script());
            Ok(())
        }
        CliAction::Completions(shell) => {
            shell.write_completions()?;
            Ok(())
        }
        CliAction::Exec(input) => please_cli::exec::run(input),
    }
}
