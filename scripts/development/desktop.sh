#!/usr/bin/env bash
# Build or run the Tauri owner app with the installed Rust toolchain.
set -euo pipefail
cd "$(dirname "$0")/../.."
source scripts/development/rust-env.sh
cd web/owner
pnpm exec tauri "$@"
