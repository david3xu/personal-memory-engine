#!/usr/bin/env bash
# Rust owns contracts; generate the static browser validator from its JSON schema.
set -euo pipefail
cd "$(dirname "$0")/../.."
source scripts/development/rust-env.sh
cargo run -p memory-engine --example export_contracts --locked
node web/owner/scripts/generate-validator.mjs
