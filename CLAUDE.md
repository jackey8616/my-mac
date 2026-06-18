# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A small, declarative bootstrap for a fresh macOS machine. A `Brewfile` lists the
software (Homebrew casks + Mac App Store apps), and `bootstrap.sh` applies it:
installs Homebrew if needed, runs `brew bundle`, and triggers the Karabiner
complex-modification imports. `install.sh` is the curl-able one-liner entry point
that provisions the essentials first (Xcode CLT, Rosetta, Homebrew), clones the
repo, then hands off to `bootstrap.sh`. No compiled code — just a Brewfile and
two shell scripts.

> History: this repo was previously a Rust CLI (`my-mac`) with a trait-layered
> installer architecture. It was rewritten to the Brewfile + bootstrap model
> because that is the standard, idempotent, and reliable way to bootstrap macOS,
> and the Rust version had bugs that broke it on a clean machine.

## Commands

```bash
# Remote one-liner (clean Mac): provisions essentials, clones repo, runs bootstrap
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/jackey8616/my-mac/main/install.sh)"

./install.sh                         # same, when the repo is already on disk
./bootstrap.sh                       # software only (installs Homebrew + everything in Brewfile)
brew bundle --file=Brewfile          # (re)apply just the Brewfile; idempotent
brew bundle check --file=Brewfile    # report what's missing without installing
brew bundle list  --file=Brewfile    # list what the Brewfile would install
shellcheck install.sh bootstrap.sh   # lint the scripts (CI runs this)
```

CI (`.github/workflows/lint.yml`) runs ShellCheck repo-wide via
`ludeeus/action-shellcheck` (it lints `*.zsh` too, not just `*.sh`), excluding
the `shell/` dir — that holds zsh config it can't parse (`ignore_paths: shell`).
Keep the scripts ShellCheck-clean.

## Layout

- **`install.sh`** — the remote entry point, meant to be run via the curl one-liner
  on a clean machine. Provisions essentials in order — Xcode Command Line Tools
  (waits in a poll loop for the GUI install), Rosetta 2 on `arm64`, Homebrew — then
  clones/updates the repo into `$MY_MAC_DIR` (default `~/.my-mac`) and `exec`s
  `bootstrap.sh`. It deliberately keeps stdin on the terminal (use the
  `/bin/bash -c "$(curl …)"` form, not `curl … | bash`) so prompts/sudo work.
  It clones the published `main` by default; set `MY_MAC_REF=<branch-or-tag>` to
  install a different ref (used to test a branch before merging). The new files
  must be on the remote for this to work — `install.sh` only fetches what's
  pushed, so a local-only branch won't appear in the `~/.my-mac` checkout.
- **`Brewfile`** — the source of truth for *what* gets installed. To add or remove
  software, edit this file. Entries: `brew "<formula>"`, `cask "<cask>"`,
  `mas "<name>", id: <app-store-id>`. Homebrew itself is intentionally not listed
  (it's the prerequisite `bootstrap.sh` installs first).
- **`bootstrap.sh`** — the orchestrator (*how* it's installed). Idempotent and safe
  to re-run. Flow: install Homebrew if missing → put `brew` on PATH
  (`/opt/homebrew` then `/usr/local`) → nudge to sign in to the App Store →
  `brew bundle` → `brew pin` every installed Brewfile formula/cask so re-runs and
  `brew upgrade` don't bump them (`pin_brewfile()`) → set up the shell (source
  `shell/my-mac.zsh` from `~/.zshrc`, make Homebrew zsh the login shell) → sign in
  to the GitHub CLI
  (`gh auth login`, skipped if already authenticated) → open each Karabiner
  `karabiner://…import?url=…` URL → print a summary. Steps that mirror the old "optional" behavior (App Store app, Karabiner
  imports) warn-and-continue rather than abort. Each Karabiner import is skipped
  when its config is already present: `config_title()` reads the JSON's top-level
  `title` (via `jq` if available, else a `grep`/`sed` fallback) and
  `karabiner_already_imported()` greps for that title under
  `~/.config/karabiner/assets/complex_modifications/` — so re-runs don't re-prompt
  or create duplicate rule sets.
- **`shell/`** — the interactive zsh setup. `my-mac.zsh` is sourced from `~/.zshrc`
  (bootstrap appends a guarded `# >>> my-mac shell setup >>>` block with an
  absolute `source` line pointing back into the repo, so editing `my-mac.zsh`
  updates every shell). It puts Homebrew on PATH, inits the Starship prompt
  (config in `shell/starship.toml`, pointed at via `STARSHIP_CONFIG`), and sources
  `zsh-autosuggestions` then `zsh-syntax-highlighting` (highlighting **must** load
  last). It has no shebang because it's sourced, not executed; its zsh-only
  syntax (e.g. `${0:A:h}`) can't be parsed by ShellCheck, so the lint workflow
  excludes the `shell/` dir via `ignore_paths: shell`.
- **`karabiner-import-config/`** — `vim.json` and `chinese-input.json`, the Karabiner
  complex modifications. These are fetched **over HTTP** by Karabiner's import
  scheme, so they must stay at this path on the public `main` branch; the raw
  GitHub URLs are hardcoded as `KARABINER_BASE` in `bootstrap.sh`.

## Gotchas

- **Mac App Store apps need an active sign-in.** The `mas "Vimlike"` entry fails if
  you're not signed in to the App Store; `bootstrap.sh` pauses to remind you and
  treats a `brew bundle` failure as non-fatal.
- **Karabiner imports require the raw JSON to be reachable on `main`.** Renaming or
  moving `karabiner-import-config/*.json` breaks the import URLs in `bootstrap.sh`
  (and `KARABINER_BASE` would need updating to match). The skip-detection also
  relies on the imported file keeping its `title`; if detection can't determine a
  title it falls back to prompting (safe) rather than wrongly skipping.
- **Pinning freezes the installed version, not a Brewfile-declared one.** Homebrew
  is rolling-release and keeps no old-bottle archive, so the Brewfile can't request
  an exact version and a fresh machine always installs the latest. The pin step
  (`pin_brewfile()`) runs `brew pin` on each installed formula/cask so a later
  `brew upgrade` or `brew bundle` re-run won't bump them — i.e. it locks *what you
  have*, it does not make installs reproducible across machines. Release a pin with
  `brew unpin <name>`. Three caveats: casks with `auto_updates true` (e.g.
  `docker-desktop`, `visual-studio-code`) can still self-update despite the pin;
  Mac App Store apps (`mas`) can't be pinned at all (warn-and-continue); and pins
  are local machine state, not tracked in the repo. `brew pin` is idempotent, so
  re-runs are safe.
- **`gh auth login` is interactive.** The GitHub CLI sign-in step runs
  `gh auth login --git-protocol ssh` (so pushes use SSH); when already signed in
  it instead runs `gh config set git_protocol ssh` to enforce SSH. It's skipped
  when there's no `/dev/tty` (CI) unless `MY_MAC_FORCE_GH` is set, and
  warns-and-continues if login doesn't finish. Choose the `workflow` token scope
  if you'll push changes under `.github/workflows/` (otherwise such pushes are
  rejected). gh stores the token in the macOS keychain.
- **Changing the default shell needs sudo + a terminal.** The shell step adds
  Homebrew zsh to `/etc/shells` (via `sudo`) and runs `chsh`, both of which prompt.
  It's skipped when there's no `/dev/tty` (CI) unless `MY_MAC_FORCE_CHSH` is set,
  and it warns-and-continues on failure. The `~/.zshrc` edit itself is safe and
  runs everywhere. Editing `shell/my-mac.zsh` after install needs no re-run since
  `~/.zshrc` sources it by absolute path — but that path is the repo location
  (`$MY_MAC_DIR`), so moving the checkout breaks the source line.
- **Docker is `cask "docker-desktop"`** (Docker Desktop) — the macOS equivalent of
  the old Linux-only `get.docker.com` script. For CLI-only Docker, use
  `brew "docker"` instead.
- **Cask/MAS ids can be renamed upstream.** If `brew bundle` can't find a token,
  verify with `brew info <token>` / `mas search <name>` and update the Brewfile.
