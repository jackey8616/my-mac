#!/usr/bin/env bash
#
# bootstrap.sh — set up a fresh Mac with the software in ./Brewfile.
#
# Safe to run multiple times: Homebrew install is skipped if already present,
# and `brew bundle` skips anything already installed.
#
#   ./bootstrap.sh
#
set -uo pipefail

# Resolve the directory this script lives in, so it works from any cwd.
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

# Raw URLs for the Karabiner complex-modification configs kept in this repo.
# Karabiner's import scheme fetches these over HTTP, so they must stay public
# at this path on the main branch.
KARABINER_BASE="https://raw.githubusercontent.com/jackey8616/my-mac/refs/heads/main/karabiner-import-config"
KARABINER_IMPORTS=(
  "vim.json"
  "chinese-input.json"
)

# Where Karabiner-Elements stores imported complex modifications. A config is
# treated as "already imported" if a file here contains its title.
KARABINER_ASSETS="$HOME/.config/karabiner/assets/complex_modifications"

# --- output helpers ---------------------------------------------------------
if [ -t 1 ]; then
  BOLD="$(tput bold 2>/dev/null || true)"
  BLUE="$(tput setaf 4 2>/dev/null || true)"
  YELLOW="$(tput setaf 3 2>/dev/null || true)"
  GREEN="$(tput setaf 2 2>/dev/null || true)"
  RESET="$(tput sgr0 2>/dev/null || true)"
else
  BOLD="" BLUE="" YELLOW="" GREEN="" RESET=""
fi

WARNINGS=()

info() { printf '%s==>%s %s\n' "${BLUE}${BOLD}" "${RESET}" "$*"; }
warn() { printf '%s==>%s %s\n' "${YELLOW}${BOLD}" "${RESET}" "$*" >&2; WARNINGS+=("$*"); }
ok()   { printf '%s==>%s %s\n' "${GREEN}${BOLD}" "${RESET}" "$*"; }

# Wait for the user to press Enter. Reads from the controlling terminal so it
# works even when this script is piped (e.g. via the curl one-liner); a no-op
# when there is no terminal at all (CI).
pause() {
  { printf '%s' "$1" > /dev/tty; read -r _ < /dev/tty; } 2>/dev/null || true
}

# Print the top-level "title" of a Karabiner complex-modification JSON file.
config_title() {
  local file="$1"
  [ -f "$file" ] || return 0
  if command -v jq >/dev/null 2>&1; then
    jq -r '.title // empty' "$file" 2>/dev/null
  else
    grep -m1 -o '"title"[[:space:]]*:[[:space:]]*"[^"]*"' "$file" \
      | sed -E 's/^"title"[[:space:]]*:[[:space:]]*"(.*)"$/\1/'
  fi
}

# Succeed if a config with the given title is already in Karabiner's assets dir.
karabiner_already_imported() {
  local title="$1"
  [ -n "$title" ] || return 1
  [ -d "$KARABINER_ASSETS" ] || return 1
  grep -rqF -- "$title" "$KARABINER_ASSETS" 2>/dev/null
}

# --- 1. Homebrew ------------------------------------------------------------
if command -v brew >/dev/null 2>&1; then
  info "Homebrew already installed."
else
  info "Installing Homebrew..."
  NONINTERACTIVE=1 /bin/bash -c \
    "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" \
    || { warn "Homebrew install failed — cannot continue."; exit 1; }
fi

# Put brew on PATH for the rest of this run (Apple Silicon, then Intel).
if [ -x /opt/homebrew/bin/brew ]; then
  eval "$(/opt/homebrew/bin/brew shellenv)"
elif [ -x /usr/local/bin/brew ]; then
  eval "$(/usr/local/bin/brew shellenv)"
fi

command -v brew >/dev/null 2>&1 || { warn "brew not on PATH after install — cannot continue."; exit 1; }

# --- 2. App Store sign-in nudge --------------------------------------------
info "The Mac App Store app (Vimlike) requires you to be signed in to the App Store."
pause "Sign in via the App Store, then press Enter to continue (Ctrl-C to abort)... "

# --- 3. brew bundle ---------------------------------------------------------
info "Installing software from Brewfile..."
if brew bundle --file="$SCRIPT_DIR/Brewfile"; then
  ok "Brewfile applied."
else
  warn "brew bundle reported failures (commonly the App Store app when not signed in). Continuing."
fi

# --- 4. Karabiner complex-modification imports ------------------------------
if brew list --cask karabiner-elements >/dev/null 2>&1; then
  info "Setting up Karabiner complex modifications."
  for cfg in "${KARABINER_IMPORTS[@]}"; do
    title="$(config_title "$SCRIPT_DIR/karabiner-import-config/$cfg")"
    if karabiner_already_imported "$title"; then
      info "  '${title}' already imported — skipping."
      continue
    fi
    info "  Opening import for '${title:-$cfg}'. Confirm it in the Karabiner window."
    open "karabiner://karabiner/assets/complex_modifications/import?url=${KARABINER_BASE}/${cfg}" \
      || warn "Failed to open Karabiner import for ${cfg}."
    pause "  Press Enter once you've confirmed the import for '${title:-$cfg}'... "
  done
else
  warn "karabiner-elements not installed — skipping complex-modification imports."
fi

# --- 5. Summary -------------------------------------------------------------
echo
if [ "${#WARNINGS[@]}" -eq 0 ]; then
  ok "Done. Your Mac is set up."
else
  warn "Done, with ${#WARNINGS[@]} item(s) needing attention:"
  for w in "${WARNINGS[@]}"; do printf '    - %s\n' "$w" >&2; done
fi
