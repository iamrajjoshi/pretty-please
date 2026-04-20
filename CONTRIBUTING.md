# Contributing

Thanks for helping make `pls` better.

## Local setup

1. Install Rust stable.
2. Clone the repo.
3. Run:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## What to optimize for

- Keep the binary tiny and boring.
- Prefer simple cross-shell behavior over clever shell-specific hacks.
- Treat friendly error messages as part of the product.
- Preserve the shell/binary split: shell snippets fetch history, Rust owns validation and execution.

## Demo workflow

The demo asset is reproducible:

```bash
vhs demo.tape
```

The tape uses helper scripts in `demo/bin/` so it can show a "needs sudo" flow without touching real system files.

## Release notes

- Update `CHANGELOG.md`.
- Run the validation commands above.
- Push a semver tag like `v0.1.0`.
- Let `.github/workflows/release.yml` build artifacts and publish installers.

## Pull requests

- Keep scope tight.
- Add or update tests when behavior changes.
- Call out shell-specific caveats in the PR description.
