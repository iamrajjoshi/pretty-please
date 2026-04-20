use thiserror::Error;

#[derive(Debug, Error)]
pub enum PleaseError {
    #[error(
        "pls needs a command. Use `pls <command...>` or install a shell hook with `pls init <shell>`."
    )]
    MissingCommand,
    #[error("there's no previous command to rerun yet.")]
    EmptyHistory,
    #[error("couldn't parse the previous command from shell history.")]
    ParseHistory(#[source] shell_words::ParseError),
    #[error("`{0}` is a shell builtin, so sudo can't change your current shell state.")]
    Builtin(String),
    #[error("`sudo` wasn't found on PATH. Install sudo first, or use `doas` directly for now.")]
    MissingSudo,
}
