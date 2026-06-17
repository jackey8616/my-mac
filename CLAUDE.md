# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A CLI tool (`my-mac`) that bootstraps a fresh macOS machine by running a declarative list of installation steps (Homebrew formulae/casks, internet shell scripts, App Store pages, and Karabiner config imports). Rust, edition 2024.

## Commands

```bash
cargo run            # build + run the installer (the real app — it will actually install things)
cargo build          # debug build
cargo build --release  # release build (LTO=fat, stripped)
cargo test           # run all tests
cargo test <name>    # run a single test by (substring) name, e.g. cargo test test_installer_steps_fail
cargo fmt -- --check # format lint — CI fails if not formatted (uses nightly)
cargo clippy         # lint — CI runs this
```

CI (`.github/workflows/test.yml`) runs on **nightly** toolchain and gates on `cargo fmt --check`, `cargo clippy`, and `cargo tarpaulin` for coverage (uploaded to Codecov). Match nightly formatting locally to avoid CI failures.

Some tests (`http_downloader`, `command_executor::test_execute`) make real network calls or spawn real processes — they are integration-style and require connectivity.

## Architecture

The codebase is layered by responsibility, with traits as the seams that make installers testable via mock injection:

- **`models/`** — plain data. `Installation` (a named group) contains `Vec<InstallationStep>`. Each `InstallationStep` carries an `InstallationStepAction` enum variant and an `optional` flag. Built with a fluent builder style (`Installation::new(..).with_install_steps(..)`, `InstallationStep::new(..).optional()`).

- **`traits/`** — the interfaces: `Installer` (name/description/is_installed/install), `Executor<T>` (runs something, returns `T`), `Downloader` (url → file). Concrete types depend on these traits so tests can inject mocks (see `BrewFormulaInstaller`'s `MockExecutor`).

- **`core/`** — generic implementations of the traits: `CommandExecutor` (shells out via `std::process::Command`, returns stdout) and `HttpDownloader` (reqwest blocking download with progress).

- **`installers/`** — one concrete `Installer` per action type (`BrewFormulaInstaller`, `InternetScriptInstaller`, `AppleStoreInstaller`), plus `InstallationManager`, the orchestrator.

### Control flow

1. `main.rs` declaratively defines all `Installation` groups and their steps — **this is the source of truth for what gets installed**. To add/remove software, edit `main.rs`.
2. `InstallationManager::install()` iterates installations → steps, and `match`es on `InstallationStepAction` to dispatch to the right installer (or, for `BrowserOpen`, opens the URL in Safari directly).
3. If a **non-optional** step fails, `install()` aborts with an error. Optional steps (`.optional()`) swallow failures and continue. This optional/required distinction is the key behavioral contract.

### Adding a new action type

Adding a variant to `InstallationStepAction` requires a matching arm in `InstallationManager::install()`'s `match` (the compiler enforces this), and usually a new `Installer` impl in `installers/`.

## Gotchas

- `InternetScriptInstaller` downloads to `./tmp/<name>_install.sh` — the `./tmp/` directory must exist at runtime or the download fails.
- `BrewFormulaInstaller::new` runs `brew info <formula>` eagerly in its constructor and **panics** if the executor errors; it also infers cask-vs-formula by string-matching the `brew info` output.
- `AppleStoreInstaller::install` builds an `open` Command but never calls `.status()`/`.spawn()`, so it currently does not actually open the page.
- Karabiner complex-modification imports are done via `BrowserOpen` of a `karabiner://` URL pointing at the JSON files in `karabiner-import-config/`.
