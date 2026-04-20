mod bash;
mod fish;
mod nu;
mod pwsh;
mod zsh;

use std::io;

use clap::CommandFactory;
use clap_complete::generate;

use crate::cli::Cli;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Pwsh,
    Nu,
}

impl Shell {
    pub fn init_script(self) -> &'static str {
        match self {
            Self::Bash => bash::INIT,
            Self::Zsh => zsh::INIT,
            Self::Fish => fish::INIT,
            Self::Pwsh => pwsh::INIT,
            Self::Nu => nu::INIT,
        }
    }

    pub fn write_completions(self) -> io::Result<()> {
        let mut command = Cli::command();
        let bin_name = command.get_name().to_string();
        let mut stdout = io::stdout();

        match self {
            Self::Bash => generate(
                clap_complete::Shell::Bash,
                &mut command,
                bin_name,
                &mut stdout,
            ),
            Self::Zsh => generate(
                clap_complete::Shell::Zsh,
                &mut command,
                bin_name,
                &mut stdout,
            ),
            Self::Fish => generate(
                clap_complete::Shell::Fish,
                &mut command,
                bin_name,
                &mut stdout,
            ),
            Self::Pwsh => generate(
                clap_complete::Shell::PowerShell,
                &mut command,
                bin_name,
                &mut stdout,
            ),
            Self::Nu => generate(
                clap_complete_nushell::Nushell,
                &mut command,
                bin_name,
                &mut stdout,
            ),
        };

        Ok(())
    }
}
