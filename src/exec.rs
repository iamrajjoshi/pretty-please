use std::borrow::Cow;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::cli::CommandInput;
use crate::error::PleaseError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Runtime {
    pub is_root: bool,
    pub sudo_path: Option<PathBuf>,
}

impl Runtime {
    pub fn detect() -> Self {
        Self {
            is_root: is_root(),
            sudo_path: which::which("sudo").ok(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionPlan {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub note: Option<String>,
}

impl ExecutionPlan {
    pub fn argv(&self) -> Vec<String> {
        let mut argv = vec![self.program.display().to_string()];
        argv.extend(self.args.clone());
        argv
    }
}

pub fn plan(input: CommandInput, runtime: &Runtime) -> Result<ExecutionPlan, PleaseError> {
    let args = resolve_args(input)?;
    let first = args[0].clone();

    if is_shell_builtin(&first) {
        return Err(PleaseError::Builtin(first));
    }

    if first == "sudo" {
        return Ok(ExecutionPlan {
            program: PathBuf::from("sudo"),
            args: args.into_iter().skip(1).collect(),
            note: Some("note: command already starts with sudo; leaving it alone.".to_string()),
        });
    }

    if runtime.is_root {
        return Ok(ExecutionPlan {
            program: PathBuf::from(&first),
            args: args.into_iter().skip(1).collect(),
            note: Some("note: already running as root; skipping sudo.".to_string()),
        });
    }

    let sudo_path = runtime.sudo_path.clone().ok_or(PleaseError::MissingSudo)?;

    Ok(ExecutionPlan {
        program: sudo_path,
        args,
        note: None,
    })
}

pub fn run(input: CommandInput) -> Result<()> {
    let runtime = Runtime::detect();
    let plan = plan(input, &runtime)?;

    if let Some(note) = &plan.note {
        eprintln!("{}", note.yellow());
    }

    let status = spawn(&plan)?;
    std::process::exit(exit_code(status));
}

pub fn spawn(plan: &ExecutionPlan) -> Result<ExitStatus> {
    Command::new(&plan.program)
        .args(&plan.args)
        .status()
        .with_context(|| {
            format!(
                "failed to launch `{}`",
                shell_words::join(plan.argv().iter().map(String::as_str))
            )
        })
}

fn resolve_args(input: CommandInput) -> Result<Vec<String>, PleaseError> {
    match input {
        CommandInput::Explicit(args) => normalize_args(args),
        CommandInput::History(command) => normalize_history(command),
    }
}

fn normalize_args(args: Vec<String>) -> Result<Vec<String>, PleaseError> {
    if args.is_empty() || args.iter().all(|arg| arg.trim().is_empty()) {
        return Err(PleaseError::MissingCommand);
    }

    Ok(args)
}

fn normalize_history(command: String) -> Result<Vec<String>, PleaseError> {
    if command.trim().is_empty() {
        return Err(PleaseError::EmptyHistory);
    }

    let parsed = shell_words::split(&command).map_err(PleaseError::ParseHistory)?;
    if parsed.is_empty() {
        return Err(PleaseError::EmptyHistory);
    }

    Ok(parsed)
}

fn is_shell_builtin(command: &str) -> bool {
    let normalized = normalize_builtin_name(command);

    matches!(
        normalized.as_ref(),
        "." | "alias"
            | "bg"
            | "bind"
            | "builtin"
            | "cd"
            | "command"
            | "complete"
            | "compgen"
            | "declare"
            | "dirs"
            | "disown"
            | "enable"
            | "eval"
            | "exec"
            | "exit"
            | "export"
            | "fc"
            | "fg"
            | "get-history"
            | "getopts"
            | "hash"
            | "help"
            | "history"
            | "import-module"
            | "jobs"
            | "let"
            | "local"
            | "logout"
            | "popd"
            | "pushd"
            | "read"
            | "readonly"
            | "return"
            | "set"
            | "set-alias"
            | "set-item"
            | "set-location"
            | "set-variable"
            | "shift"
            | "shopt"
            | "source"
            | "trap"
            | "type"
            | "typeset"
            | "ulimit"
            | "umask"
            | "unalias"
            | "unset"
            | "wait"
    )
}

fn normalize_builtin_name(command: &str) -> Cow<'_, str> {
    if command == "." {
        Cow::Borrowed(".")
    } else {
        Cow::Owned(command.trim().to_ascii_lowercase())
    }
}

fn exit_code(status: ExitStatus) -> i32 {
    status.code().unwrap_or({
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;

            status.signal().map_or(1, |signal| 128 + signal)
        }

        #[cfg(not(unix))]
        {
            1
        }
    })
}

#[cfg(unix)]
fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

#[cfg(not(unix))]
fn is_root() -> bool {
    false
}
