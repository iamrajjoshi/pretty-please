# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Initial `pls` release with cross-shell init hooks for bash, zsh, fish, PowerShell, and Nushell.
- Safe sudo wrapping with guards for shell builtins, double-sudo, missing sudo, and root passthrough.
- Integration tests for planner behavior and shell init snapshots.
- GitHub Actions CI and `cargo-dist` release plumbing.
- Reproducible VHS demo assets, docs, and community health files.

## 0.1.0 - 2026-04-19

### Added

- First public cut of `pls`.
