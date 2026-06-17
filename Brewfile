# Brewfile — declarative list of software for a fresh Mac.
# Apply with `brew bundle` (run via ./bootstrap.sh). Idempotent: re-running
# skips anything already installed.
#
# Homebrew itself is NOT listed here — it is the prerequisite that
# bootstrap.sh installs before this file is applied.

# CLI to install Mac App Store apps (required by the `mas` entries below)
brew "mas"

# Apps (casks)
cask "docker-desktop"      # was the Linux-only get.docker.com script; Docker Desktop is the macOS equivalent
cask "karabiner-elements"  # keyboard customization manager
cask "amethyst"            # tiling window manager
cask "itsycal"             # tiny menu bar calendar

# Mac App Store apps (requires being signed in to the App Store)
mas "Vimlike", id: 1584519802  # Safari extension for vim-like browsing
