use std::path::PathBuf;

use please_cli::cli::CommandInput;
use please_cli::error::PleaseError;
use please_cli::exec::{Runtime, plan};
use please_cli::shell::Shell;

fn runtime(is_root: bool, sudo_path: Option<&str>) -> Runtime {
    Runtime {
        is_root,
        sudo_path: sudo_path.map(PathBuf::from),
    }
}

#[test]
fn prepends_sudo_for_normal_commands() {
    let plan = plan(
        CommandInput::Explicit(vec!["chmod".into(), "600".into(), "secret.txt".into()]),
        &runtime(false, Some("/usr/bin/sudo")),
    )
    .expect("command should plan");

    assert_eq!(plan.program, PathBuf::from("/usr/bin/sudo"));
    assert_eq!(plan.args, vec!["chmod", "600", "secret.txt"]);
    assert_eq!(plan.note, None);
}

#[test]
fn leaves_existing_sudo_alone() {
    let plan = plan(
        CommandInput::Explicit(vec!["sudo".into(), "chmod".into(), "600".into()]),
        &runtime(false, Some("/usr/bin/sudo")),
    )
    .expect("command should plan");

    assert_eq!(plan.program, PathBuf::from("sudo"));
    assert_eq!(plan.args, vec!["chmod", "600"]);
    assert!(
        plan.note
            .as_deref()
            .is_some_and(|note| note.contains("already starts with sudo"))
    );
}

#[test]
fn skips_sudo_for_root() {
    let plan = plan(
        CommandInput::Explicit(vec!["chmod".into(), "600".into(), "secret.txt".into()]),
        &runtime(true, Some("/usr/bin/sudo")),
    )
    .expect("command should plan");

    assert_eq!(plan.program, PathBuf::from("chmod"));
    assert_eq!(plan.args, vec!["600", "secret.txt"]);
    assert!(
        plan.note
            .as_deref()
            .is_some_and(|note| note.contains("already running as root"))
    );
}

#[test]
fn rejects_shell_builtins() {
    let error = plan(
        CommandInput::Explicit(vec!["cd".into(), "/root".into()]),
        &runtime(false, Some("/usr/bin/sudo")),
    )
    .expect_err("builtins should be rejected");

    assert!(matches!(error, PleaseError::Builtin(command) if command == "cd"));
}

#[test]
fn rejects_blank_history_commands() {
    let error = plan(
        CommandInput::History("   ".into()),
        &runtime(false, Some("/usr/bin/sudo")),
    )
    .expect_err("blank history should error");

    assert!(matches!(error, PleaseError::EmptyHistory));
}

#[test]
fn bash_init_snapshot() {
    assert_eq!(
        Shell::Bash.init_script(),
        r#"pls() {
  if [ "$#" -eq 0 ]; then
    local previous
    previous="$(history -p '!!' 2>/dev/null)" || {
      command pls --history-command ""
      return $?
    }

    if [[ "$previous" != *[![:space:]]* ]]; then
      command pls --history-command "$previous"
      return $?
    fi

    builtin eval "command pls ${previous}"
  else
    command pls "$@"
  fi
}
"#
    );
}

#[test]
fn zsh_init_snapshot() {
    assert_eq!(
        Shell::Zsh.init_script(),
        r#"pls() {
  if [ "$#" -eq 0 ]; then
    local previous
    previous="$(fc -ln -1 2>/dev/null)" || {
      command pls --history-command ""
      return $?
    }

    if [[ "$previous" != *[![:space:]]* ]]; then
      command pls --history-command "$previous"
      return $?
    fi

    builtin eval "command pls ${previous}"
  else
    command pls "$@"
  fi
}
"#
    );
}

#[test]
fn fish_init_snapshot() {
    assert_eq!(
        Shell::Fish.init_script(),
        r#"function pls --description 'Re-run the previous command with sudo'
    if test (count $argv) -eq 0
        set -l previous (history --max=1)

        if test -z (string trim -- $previous)
            command pls --history-command "$previous"
            return $status
        end

        eval "command pls $previous"
    else
        command pls $argv
    end
end
"#
    );
}

#[test]
fn pwsh_init_snapshot() {
    assert_eq!(
        Shell::Pwsh.init_script(),
        r#"function pls {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]] $Args
    )

    $pls_bin = Get-Command pls -CommandType Application | Select-Object -First 1 -ExpandProperty Source
    if (-not $pls_bin) {
        Write-Error "pls binary not found in PATH."
        return
    }

    if ($Args.Count -eq 0) {
        $previous = (Get-History -Count 1).CommandLine
        if ([string]::IsNullOrWhiteSpace($previous)) {
            & $pls_bin --history-command $previous
            return
        }

        Invoke-Expression "& '$pls_bin' $previous"
    } else {
        & $pls_bin @Args
    }
}
"#
    );
}

#[test]
fn nu_init_snapshot() {
    assert_eq!(
        Shell::Nu.init_script(),
        r#"def --wrapped pls [...args: string] {
  if ($args | is-empty) {
    let previous = (history | last 1 | get command | first | default "")

    if (($previous | str trim) | is-empty) {
      ^pls --history-command $previous
    } else {
      nu -c $'^pls ($previous)'
    }
  } else {
    ^pls ...$args
  }
}
"#
    );
}
