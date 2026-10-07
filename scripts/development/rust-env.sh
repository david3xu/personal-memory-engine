#!/usr/bin/env bash
# Locate the installed Rust toolchain without changing global shell configuration.
set -euo pipefail
if ! command -v cargo >/dev/null 2>&1; then
  rust_bin="$(dirname "$(rustup which cargo)")"
  export PATH="$rust_bin:$PATH"
fi
