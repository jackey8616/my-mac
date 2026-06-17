#!/usr/bin/env bash
#
# verify.sh — check that everything in the Brewfile is actually installed.
#
# Reads the Brewfile (the source of truth) and confirms each entry:
#   - brew  "<formula>"  -> brew list --formula <formula>
#   - cask  "<cask>"     -> brew list --cask <cask>
#   - mas   "<n>", id: N -> mas list contains N
#
# Mac App Store apps need an App Store sign-in, which isn't available in CI, so
# those checks are reported as skipped when running under CI/GitHub Actions.
#
# Exits non-zero if any required item is missing. Useful both as the CI
# assertion and as a post-install check you can run yourself.
#
set -uo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
BREWFILE="${1:-$SCRIPT_DIR/Brewfile}"

if [ -t 1 ]; then
  GREEN="$(tput setaf 2 2>/dev/null || true)"
  RED="$(tput setaf 1 2>/dev/null || true)"
  YELLOW="$(tput setaf 3 2>/dev/null || true)"
  RESET="$(tput sgr0 2>/dev/null || true)"
else
  GREEN="" RED="" YELLOW="" RESET=""
fi

[ -f "$BREWFILE" ] || { printf 'verify: Brewfile not found at %s\n' "$BREWFILE" >&2; exit 2; }
command -v brew >/dev/null 2>&1 || { printf 'verify: Homebrew is not installed\n' >&2; exit 2; }

PASS=0 FAIL=0 SKIP=0
pass() { printf '  %s✓%s %s\n' "$GREEN" "$RESET" "$1"; PASS=$((PASS + 1)); }
fail() { printf '  %s✗%s %s — %s\n' "$RED" "$RESET" "$1" "$2"; FAIL=$((FAIL + 1)); }
skip() { printf '  %s•%s %s — %s\n' "$YELLOW" "$RESET" "$1" "$2"; SKIP=$((SKIP + 1)); }

# Running under CI? (no App Store sign-in available there)
is_ci() { [ -n "${CI:-}" ] || [ -n "${GITHUB_ACTIONS:-}" ]; }

# --- formulae ---------------------------------------------------------------
while IFS= read -r name; do
  [ -n "$name" ] || continue
  if brew list --formula "$name" >/dev/null 2>&1; then
    pass "formula $name"
  else
    fail "formula $name" "not installed"
  fi
done < <(sed -nE 's/^[[:space:]]*brew[[:space:]]+"([^"]+)".*/\1/p' "$BREWFILE")

# --- casks ------------------------------------------------------------------
while IFS= read -r name; do
  [ -n "$name" ] || continue
  if brew list --cask "$name" >/dev/null 2>&1; then
    pass "cask $name"
  else
    fail "cask $name" "not installed"
  fi
done < <(sed -nE 's/^[[:space:]]*cask[[:space:]]+"([^"]+)".*/\1/p' "$BREWFILE")

# --- Mac App Store apps -----------------------------------------------------
while IFS= read -r entry; do
  [ -n "$entry" ] || continue
  id="${entry%%|*}"
  app="${entry#*|}"
  if mas list 2>/dev/null | grep -q "^${id}[[:space:]]"; then
    pass "mas $app ($id)"
  elif is_ci; then
    skip "mas $app ($id)" "App Store sign-in unavailable in CI"
  else
    fail "mas $app ($id)" "not installed (sign in to the App Store, then re-run bootstrap)"
  fi
done < <(sed -nE 's/^[[:space:]]*mas[[:space:]]+"([^"]+)".*id:[[:space:]]*([0-9]+).*/\2|\1/p' "$BREWFILE")

echo
printf 'verify: %s%d passed%s, %s%d failed%s, %d skipped\n' \
  "$GREEN" "$PASS" "$RESET" "$RED" "$FAIL" "$RESET" "$SKIP"
[ "$FAIL" -eq 0 ]
