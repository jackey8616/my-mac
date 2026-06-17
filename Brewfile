# Brewfile — declarative list of software for a fresh Mac.
# Apply with `brew bundle` (run via ./bootstrap.sh). Idempotent: re-running
# skips anything already installed.
#
# Homebrew itself is NOT listed here — it is the prerequisite that
# bootstrap.sh installs before this file is applied.

# CLI to install Mac App Store apps (required by the `mas` entries below)
brew "mas"

# GitHub CLI — PRs/issues from the terminal; also a git auth/credential helper.
# bootstrap.sh signs you in (`gh auth login`) if you aren't already.
brew "gh"

# Shell (zsh + niceties). zsh is macOS's default; the Homebrew build is newer and
# bootstrap.sh makes it the login shell. The plugins and prompt are wired up in
# shell/my-mac.zsh, which bootstrap.sh sources from ~/.zshrc.
brew "zsh"                      # newer zsh than the system one
brew "zsh-autosuggestions"     # fish-style "ghost text" suggestions from history
brew "zsh-syntax-highlighting" # colourises the command line as you type
brew "starship"                # cross-shell prompt (git, versions, exit codes)

# Apps (casks)
cask "docker-desktop"      # was the Linux-only get.docker.com script; Docker Desktop is the macOS equivalent
cask "karabiner-elements"  # keyboard customization manager
cask "amethyst"            # tiling window manager
cask "itsycal"             # tiny menu bar calendar
cask "visual-studio-code"  # code editor
cask "tableplus"           # database GUI client
cask "discord"             # voice and text chat

# Mac App Store apps (requires being signed in to the App Store)
mas "Vimlike", id: 1584519802  # Safari extension for vim-like browsing
