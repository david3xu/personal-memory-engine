#!/usr/bin/env bash
# Locate the installed macOS app; keep all protocol traffic on stdout.
set -euo pipefail
helper_path="${PERSONAL_MEMORY_HELPER:-}"
if [[ -z "$helper_path" ]]; then
  for app_path in "$HOME/Applications/Personal Memory Engine.app" "/Applications/Personal Memory Engine.app"; do
    candidate="$app_path/Contents/MacOS/memory-mcp"
    if [[ -x "$candidate" ]]; then helper_path="$candidate"; break; fi
  done
fi
if [[ -z "$helper_path" || ! -x "$helper_path" ]]; then
  echo 'Install Personal Memory Engine in Applications before connecting this desktop-only plugin.' >&2
  exit 1
fi
if [[ -n "${PERSONAL_MEMORY_DATA_DIR:-}" ]]; then
  exec "$helper_path" --data-dir "$PERSONAL_MEMORY_DATA_DIR"
else
  exec "$helper_path"
fi
