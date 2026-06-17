# my-mac.zsh — interactive zsh setup, sourced from ~/.zshrc by bootstrap.sh.
#
# Provides: Homebrew on PATH, the Starship prompt, history autosuggestions, and
# command-line syntax highlighting. This file lives in the repo — edit it here
# and the changes apply on your next shell (~/.zshrc just sources it).
#
# NOTE: this is zsh, not POSIX sh. It has no shebang because it's sourced, not
# executed, and its zsh-only syntax (e.g. ${0:A:h}) can't be parsed by ShellCheck
# — so the lint workflow excludes the shell/ dir (ignore_paths in
# .github/workflows/lint.yml).

# Directory this file lives in. zsh sets $0 to the sourced file's path, and the
# :A:h modifiers turn it into an absolute directory.
MY_MAC_SHELL_DIR="${0:A:h}"

# Homebrew prefix. Resolve it without the slow `brew --prefix` subprocess so we
# don't add latency to every shell start.
if [[ -z "${HOMEBREW_PREFIX:-}" ]]; then
  if [[ -x /opt/homebrew/bin/brew ]]; then
    HOMEBREW_PREFIX=/opt/homebrew      # Apple Silicon
  elif [[ -x /usr/local/bin/brew ]]; then
    HOMEBREW_PREFIX=/usr/local         # Intel
  fi
fi

# Put Homebrew and its installed tools on PATH (idempotent if already done).
if [[ -n "${HOMEBREW_PREFIX:-}" && -x "$HOMEBREW_PREFIX/bin/brew" ]]; then
  eval "$("$HOMEBREW_PREFIX/bin/brew" shellenv)"
fi

# --- Starship prompt --------------------------------------------------------
# Config is kept in the repo (shell/starship.toml) so the prompt is reproducible.
if command -v starship >/dev/null 2>&1; then
  export STARSHIP_CONFIG="$MY_MAC_SHELL_DIR/starship.toml"
  eval "$(starship init zsh)"
fi

# --- Autosuggestions --------------------------------------------------------
if [[ -r "$HOMEBREW_PREFIX/share/zsh-autosuggestions/zsh-autosuggestions.zsh" ]]; then
  source "$HOMEBREW_PREFIX/share/zsh-autosuggestions/zsh-autosuggestions.zsh"
fi

# --- Syntax highlighting ----------------------------------------------------
# Must be sourced LAST — it hooks the line editor and expects to wrap everything
# else, so anything that touches the prompt/widgets should already be set up.
if [[ -r "$HOMEBREW_PREFIX/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh" ]]; then
  source "$HOMEBREW_PREFIX/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
fi
