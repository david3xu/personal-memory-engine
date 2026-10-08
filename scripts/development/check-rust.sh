#!/usr/bin/env bash
# Verify engine/runtime formatting, warnings, and behavior independently of the desktop host.
set -euo pipefail
cd "$(dirname "$0")/../.."
source scripts/development/rust-env.sh
node scripts/development/check-boundaries.mjs
cargo fmt --all -- --check
cargo clippy -p memory-engine -p memory-local-runtime -p memory-public-snapshot -p memory-github-publication --all-targets --locked -- -D warnings
cargo test -p memory-engine -p memory-local-runtime -p memory-public-snapshot -p memory-github-publication --locked
cargo run -p memory-engine --example export_contracts --locked -- --check
