#!/usr/bin/env bash
# Build/run a desktop bundle with its standalone MCP helper, never a source-path dependency.
set -euo pipefail
cd "$(dirname "$0")/../.."
source scripts/development/rust-env.sh
profile=debug
cargo_args=()
if [[ "${1:-}" == build ]] && [[ " $* " != *" --debug "* ]]; then
  profile=release
  cargo_args+=(--release)
fi
cargo build -p memory-local-runtime --bin memory-mcp --locked "${cargo_args[@]}"
target_triple="$(rustc -vV | sed -n 's/^host: //p')"
mkdir -p web/owner/src-tauri/binaries
cp "target/$profile/memory-mcp" "web/owner/src-tauri/binaries/memory-mcp-$target_triple"
cd web/owner
pnpm exec tauri "$@"
