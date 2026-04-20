use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::shell::Shell;

#[derive(Debug, Parser)]
#[command(
    name = "pls",
    bin_name = "pls",
    version,
    about = "The polite sudo.",
    long_about = None,
    override_usage = "pls [ARGS...]\n       pls init <SHELL>\n       pls completions <SHELL>",
    after_help = "Examples:\n  pls chmod 600 secrets.txt\n  pls init zsh\n  pls completions fish",
    disable_help_subcommand = true,
    propagate_version = true
)]
pub struct Cli {
    /// Raw command captured by a shell hook. Hidden because it is only meant for shell integrations.
    #[arg(long, hide = true, env = "PLS_HISTORY_COMMAND")]
    pub history_command: Option<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print the shell hook for your shell.
    Init(InitArgs),
    /// Generate completion scripts for your shell.
    Completions(CompletionArgs),
    #[command(external_subcommand)]
    Exec(Vec<String>),
}

#[derive(Debug, Args)]
pub struct InitArgs {
    pub shell: Shell,
}

#[derive(Debug, Args)]
pub struct CompletionArgs {
    pub shell: Shell,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandInput {
    History(String),
    Explicit(Vec<String>),
}

impl Cli {
    pub fn into_command_input(self) -> CliAction {
        match self.command {
            Some(Command::Init(args)) => CliAction::Init(args.shell),
            Some(Command::Completions(args)) => CliAction::Completions(args.shell),
            Some(Command::Exec(args)) => CliAction::Exec(CommandInput::Explicit(args)),
            None => match self.history_command {
                Some(command) => CliAction::Exec(CommandInput::History(command)),
                None => CliAction::Exec(CommandInput::Explicit(Vec::new())),
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliAction {
    Init(Shell),
    Completions(Shell),
    Exec(CommandInput),
}

impl ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[Shell::Bash, Shell::Zsh, Shell::Fish, Shell::Pwsh, Shell::Nu]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Shell::Bash => clap::builder::PossibleValue::new("bash"),
            Shell::Zsh => clap::builder::PossibleValue::new("zsh"),
            Shell::Fish => clap::builder::PossibleValue::new("fish"),
            Shell::Pwsh => clap::builder::PossibleValue::new("pwsh").alias("powershell"),
            Shell::Nu => clap::builder::PossibleValue::new("nu").alias("nushell"),
        })
    }
}
