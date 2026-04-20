<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/logo-dark.svg">
    <img src="assets/logo.svg" alt="pls" width="420">
  </picture>

  <p><strong>the polite sudo</strong></p>

  <p>
    <a href="https://github.com/iamrajjoshi/pls/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/iamrajjoshi/pls/ci.yml?branch=main&label=ci"></a>
    <a href="https://github.com/iamrajjoshi/pls/releases"><img alt="Release" src="https://img.shields.io/github/v/release/iamrajjoshi/pls?display_name=tag"></a>
    <a href="https://crates.io/crates/please-cli"><img alt="crates.io" src="https://img.shields.io/crates/v/please-cli.svg"></a>
    <a href="LICENSE-MIT"><img alt="License" src="https://img.shields.io/badge/license-MIT%20or%20Apache--2.0-blue"></a>
  </p>
</div>

`pls` reruns your previous shell command with `sudo` prepended, while keeping the shell-specific history lookup where it belongs: in the shell you are currently using.

![pls demo](assets/demo.gif)

## Install

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/iamrajjoshi/pls/releases/latest/download/please-cli-installer.sh | sh
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/iamrajjoshi/pls/releases/latest/download/please-cli-installer.ps1 | iex"
```

| Method | Command | Status |
| --- | --- | --- |
| Homebrew | `brew install iamrajjoshi/tap/pls` | wired through `cargo-dist` |
| Cargo | `cargo install please-cli --bin pls` | available |
| Shell installer | `curl ... | sh` | generated on release |
| PowerShell installer | `irm ... \| iex` | generated on release |
| Nix | `nix run github:iamrajjoshi/pls` | included via `flake.nix` |
| AUR | `yay -S pls-git` | included as `packaging/aur/PKGBUILD` |
| Scoop | community manifest welcome | not automated yet |

## Setup

Hook `pls` into your shell once so the zero-argument form can see your live, in-session command history.

<details>
<summary><strong>zsh</strong></summary>

```sh
echo 'eval "$(pls init zsh)"' >> ~/.zshrc
```
</details>

<details>
<summary><strong>bash</strong></summary>

```sh
echo 'eval "$(pls init bash)"' >> ~/.bashrc
```
</details>

<details>
<summary><strong>fish</strong></summary>

```fish
pls init fish | source
```
</details>

<details>
<summary><strong>PowerShell</strong></summary>

```powershell
Add-Content $PROFILE 'Invoke-Expression (& pls init pwsh)'
```
</details>

<details>
<summary><strong>Nushell</strong></summary>

```nu
pls init nu | save --append ~/.config/nushell/config.nu
```
</details>

## Usage

Rerun the last command with `sudo`:

```console
$ mkdir /var/my-app
mkdir: /var/my-app: Permission denied
$ pls
```

Wrap an explicit command directly:

```console
$ pls systemctl restart nginx
```

Print shell completions:

```console
$ pls completions zsh > _pls
```

## Compatibility

| Shell | macOS | Linux | Windows |
| --- | --- | --- | --- |
| bash | yes | yes | via WSL/Git Bash |
| zsh | yes | yes | via WSL |
| fish | yes | yes | via WSL |
| pwsh | yes | yes | yes |
| nu | yes | yes | yes |

`pls` assumes a `sudo` executable exists on `PATH`. If it does not, `pls` fails fast with a friendly error instead of guessing.

## Why This Split Exists

The shell function handles one thing only: reading the most recent in-memory command from your current shell session. The Rust binary handles everything portable:

- refusing shell builtins like `cd` or `export`
- avoiding `sudo sudo ...`
- skipping `sudo` when already root
- printing init snippets and completions
- staying tiny enough to ship as a single CLI binary

## Comparison

| Tool | What it does well | Why `pls` exists |
| --- | --- | --- |
| `sudo !!` | tiny bash/zsh trick | shell-specific, brittle outside history-expansion shells |
| shell alias/function only | zero dependencies | hard to test, hard to distribute, hard to support across shells |
| `thefuck` | broad command correction | much heavier than a single "rerun with sudo" primitive |
| `pls` | one job, cross-shell, tiny binary | purpose-built for "that should have been sudo" |

## FAQ

**Does `pls` work with pipes and redirects?**  
Mostly the same way a manual `sudo <previous command>` does: `sudo` only applies to the leftmost command in a pipeline. A shell-wrapped mode can come later.

**Why not just alias `sudo !!`?**  
Because `!!` is not portable, history access differs wildly between shells, and distributing a tested cross-shell tool is nicer than copy-pasting five different dotfile snippets.

**Why is the crate called `please-cli` if the binary is `pls`?**  
Because both `pls` and `please` are already taken on crates.io. The published package name stays releaseable while the installed binary remains `pls`.

## Development

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

The demo GIF is generated from [`demo.tape`](demo.tape) with [VHS](https://github.com/charmbracelet/vhs), and the release pipeline is powered by `cargo-dist`.

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
