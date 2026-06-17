# Clode's Mac

[![Lint](https://github.com/jackey8616/my-mac/actions/workflows/lint.yml/badge.svg)](https://github.com/jackey8616/my-mac/actions/workflows/lint.yml)

Bootstrap a fresh Mac with my software, declaratively. A [`Brewfile`](./Brewfile)
lists everything; [`bootstrap.sh`](./bootstrap.sh) installs Homebrew and applies it.
Idempotent — safe to run again any time.

## Usage

### Quick start (one line, recommended)

On a brand-new Mac, paste this into Terminal — no prerequisites:

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/jackey8616/my-mac/main/install.sh)"
```

[`install.sh`](./install.sh) takes a completely clean Mac and handles the essentials for you:

1. Installs the **Xcode Command Line Tools** (git, compilers) if missing.
2. Installs **Rosetta 2** on Apple Silicon (for Intel-only apps).
3. Installs **[Homebrew](https://brew.sh/)** if missing.
4. Clones this repo to `~/.my-mac` (override the location with `MY_MAC_DIR`).
5. Runs [`bootstrap.sh`](./bootstrap.sh), which installs everything in the
   `Brewfile`, sets up the shell (zsh + Starship), and imports the Karabiner
   configs.

> **Sign in to the App Store first.** The `Vimlike` extension installs via the Mac
> App Store (`mas`), which needs you signed in. The script pauses to remind you.

During the run it also sets up your **shell**: Homebrew's zsh becomes your login
shell (you'll be prompted for your password by `chsh`), and `~/.zshrc` gets a
small managed block that loads the [Starship](https://starship.rs/) prompt plus
`zsh-autosuggestions` and `zsh-syntax-highlighting`. Open a new terminal
afterwards to pick it up.

Afterwards, a couple of apps need a one-time approval in **System Settings**:
Karabiner-Elements asks you to allow its driver extension and Input Monitoring,
and Docker Desktop asks for a privileged helper on first launch.

### From a clone

If you'd rather clone it yourself (git required) and run the steps directly:

```sh
git clone https://github.com/jackey8616/my-mac.git
cd my-mac
./bootstrap.sh
```

`bootstrap.sh` installs Homebrew if needed, then applies the `Brewfile` and the
Karabiner configs (steps 3–5 above). Use this when you already have a checkout.

### Re-running

The whole script is **idempotent** — run it again any time:

- Homebrew install is skipped when `brew` is already present.
- `brew bundle` installs only what's missing.
- Karabiner imports are skipped automatically for any config already imported
  (detected by title under `~/.config/karabiner/assets/complex_modifications/`),
  so a re-run won't re-prompt or create duplicate rule sets.

To re-apply just the software list (no Karabiner prompts at all):

```sh
brew bundle --file=Brewfile          # install anything missing
brew bundle check --file=Brewfile    # see what's missing without installing
```

## Install list

Defined in [`Brewfile`](./Brewfile):

- [Homebrew](https://brew.sh/) — installed by `bootstrap.sh` (prerequisite)
- Shell — `zsh` (Homebrew build, set as the login shell) with:
  - [Starship](https://starship.rs/) prompt — `brew "starship"`, config in [`shell/starship.toml`](./shell/starship.toml)
  - [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions) — `brew "zsh-autosuggestions"`
  - [zsh-syntax-highlighting](https://github.com/zsh-users/zsh-syntax-highlighting) — `brew "zsh-syntax-highlighting"`
  - wired up in [`shell/my-mac.zsh`](./shell/my-mac.zsh), sourced from `~/.zshrc`
- [Docker Desktop](https://www.docker.com/products/docker-desktop/) — `cask "docker-desktop"`
- [Karabiner-Elements](https://karabiner-elements.pqrs.org/) — `cask "karabiner-elements"`
  - Plugin: [Vim Mode Plus](https://github.com/jonasdiemer/karabiner-vim-mode-plus) — imported from [`karabiner-import-config/vim.json`](./karabiner-import-config/vim.json)
  - Plugin: Chinese-English switch — imported from [`karabiner-import-config/chinese-input.json`](./karabiner-import-config/chinese-input.json) (modified version stored in this repo)
- [Amethyst](https://github.com/ianyh/Amethyst) — `cask "amethyst"`
- [Itsycal](https://www.mowglii.com/itsycal/) — `cask "itsycal"`
- [Visual Studio Code](https://code.visualstudio.com/) — `cask "visual-studio-code"`
- [TablePlus](https://tableplus.com/) — `cask "tableplus"`
- [Discord](https://discord.com/) — `cask "discord"`
- [Vimlike](https://apps.apple.com/app/id1584519802) — Safari extension, via Mac App Store (`mas`)

To change what gets installed, edit the `Brewfile`.

## License

MIT
