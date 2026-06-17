#!/usr/bin/env bash
#
# install.sh — one-line bootstrap for a fresh Mac.
#
# Designed to be run straight from the network, Homebrew-style:
#
#   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/jackey8616/my-mac/main/install.sh)"
#
# It ensures the essentials are present (Xcode Command Line Tools, Rosetta 2 on
# Apple Silicon, Homebrew), fetches this repo, then hands off to bootstrap.sh,
# which installs the software in the Brewfile and the Karabiner configs.
#
# Idempotent and safe to re-run. Override the checkout location with MY_MAC_DIR.
#
set -uo pipefail

REPO_URL="https://github.com/jackey8616/my-mac.git"
TARBALL_URL="https://github.com/jackey8616/my-mac/archive/refs/heads/main.tar.gz"
TARGET_DIR="${MY_MAC_DIR:-$HOME/.my-mac}"

# --- output helpers ---------------------------------------------------------
if [ -t 1 ]; then
  BOLD="$(tput bold 2>/dev/null || true)"
  BLUE="$(tput setaf 4 2>/dev/null || true)"
  YELLOW="$(tput setaf 3 2>/dev/null || true)"
  GREEN="$(tput setaf 2 2>/dev/null || true)"
  RED="$(tput setaf 1 2>/dev/null || true)"
  RESET="$(tput sgr0 2>/dev/null || true)"
else
  BOLD="" BLUE="" YELLOW="" GREEN="" RED="" RESET=""
fi

info() { printf '%s==>%s %s\n' "${BLUE}${BOLD}" "${RESET}" "$*"; }
warn() { printf '%s==>%s %s\n' "${YELLOW}${BOLD}" "${RESET}" "$*" >&2; }
ok()   { printf '%s==>%s %s\n' "${GREEN}${BOLD}" "${RESET}" "$*"; }
die()  { printf '%s==>%s %s\n' "${RED}${BOLD}" "${RESET}" "$*" >&2; exit 1; }

# --- 0. macOS only ----------------------------------------------------------
[ "$(uname -s)" = "Darwin" ] || die "This installer only supports macOS."

# --- 1. Xcode Command Line Tools (provides git, compilers) ------------------
if xcode-select -p >/dev/null 2>&1; then
  info "Xcode Command Line Tools already installed."
else
  info "Installing Xcode Command Line Tools..."
  warn "A dialog will appear — click \"Install\" and accept the license. This will continue automatically when it finishes."
  xcode-select --install >/dev/null 2>&1 || true
  # Wait for the GUI install to complete.
  until xcode-select -p >/dev/null 2>&1; do
    sleep 5
  done
  ok "Xcode Command Line Tools installed."
fi

# --- 2. Rosetta 2 (Apple Silicon, for Intel-only apps) ----------------------
if [ "$(uname -m)" = "arm64" ]; then
  if arch -x86_64 /usr/bin/true >/dev/null 2>&1; then
    info "Rosetta 2 already available."
  else
    info "Installing Rosetta 2..."
    softwareupdate --install-rosetta --agree-to-license || warn "Rosetta 2 install failed; continuing."
  fi
fi

# --- 3. Homebrew ------------------------------------------------------------
if command -v brew >/dev/null 2>&1; then
  info "Homebrew already installed."
else
  info "Installing Homebrew..."
  NONINTERACTIVE=1 /bin/bash -c \
    "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" \
    || die "Homebrew install failed."
fi

# Put brew on PATH for the rest of this run (Apple Silicon, then Intel).
if [ -x /opt/homebrew/bin/brew ]; then
  eval "$(/opt/homebrew/bin/brew shellenv)"
elif [ -x /usr/local/bin/brew ]; then
  eval "$(/usr/local/bin/brew shellenv)"
fi
command -v brew >/dev/null 2>&1 || die "brew not on PATH after install."

# --- 4. Fetch this repo -----------------------------------------------------
if [ -d "$TARGET_DIR/.git" ]; then
  info "Updating existing checkout at $TARGET_DIR..."
  git -C "$TARGET_DIR" pull --ff-only || warn "Could not update $TARGET_DIR; using the existing copy."
elif command -v git >/dev/null 2>&1; then
  info "Cloning my-mac into $TARGET_DIR..."
  git clone --depth 1 "$REPO_URL" "$TARGET_DIR" || die "git clone failed."
else
  info "Downloading my-mac into $TARGET_DIR..."
  mkdir -p "$TARGET_DIR"
  curl -fsSL "$TARBALL_URL" | tar -xz -C "$TARGET_DIR" --strip-components=1 \
    || die "Download failed."
fi

# --- 5. Hand off to bootstrap.sh -------------------------------------------
[ -f "$TARGET_DIR/bootstrap.sh" ] || die "bootstrap.sh not found in $TARGET_DIR."
info "Running bootstrap..."
exec /bin/bash "$TARGET_DIR/bootstrap.sh"
