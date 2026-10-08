#!/usr/bin/env bash
# Package the verified desktop bundle into a headless, drag-to-Applications preview disk image.
set -euo pipefail
cd "$(dirname "$0")/../.."
if [[ "$(uname -s)" != Darwin ]]; then
  echo 'macOS preview packaging requires macOS.' >&2
  exit 1
fi
bash scripts/development/desktop.sh build
source scripts/development/rust-env.sh
app_bundle="$PWD/target/release/bundle/macos/Personal Memory Engine.app"
MEMORY_PACKAGE_SOURCE="$app_bundle/Contents/Resources/plugin-source" MEMORY_PACKAGE_HELPER="$app_bundle/Contents/MacOS/memory-mcp" cargo test -p memory-local-runtime --test worker_package --locked
MCP_HELPER="$app_bundle/Contents/MacOS/memory-mcp" pnpm test:package
image_dir="$PWD/target/release/bundle/dmg"
mkdir -p "$image_dir"
image_name="Personal-Memory-Engine-macos-$(uname -m)-preview.dmg"
image_path="$image_dir/$image_name"
image_stage="$(mktemp -d "${TMPDIR:-/tmp}/personal-memory-image.XXXXXX")"
trap 'rm -rf "$image_stage"' EXIT
ditto "$app_bundle" "$image_stage/Personal Memory Engine.app"
ln -s /Applications "$image_stage/Applications"
cat > "$image_stage/INSTALL.txt" <<'GUIDE'
Personal Memory Engine — unsigned testing preview

Drag Personal Memory Engine.app onto Applications, then open it.
Choose Settings, Connect once, restart ChatGPT once, then open a local test chat.
Review the sample choice and press Send in ChatGPT.
The app reports Recording verified after that choice is saved locally.

Requires macOS Apple Silicon for the arm64 package and a desktop host/account
with a compatible local Codex host and bundled MCP connection manager. No developer runtime is required.

This package is not signed or notarized. Normal public installation still needs
verification. The owner’s Work recording test passed on the maintainer setup;
compatibility with other accounts/modes is not guaranteed. Do not disable system security.
Share decisions publishes only a reviewed selection to your GitHub Pages repository.
Backup/restore is not included in this early preview.

Guide: https://github.com/david3xu/personal-memory-engine/blob/634577aca2adf0b2f96876b1f187f0281b4d2cd4/docs/user-guides/install-and-connect.md
GUIDE
hdiutil create -ov -volname 'Personal Memory Engine Preview' -srcfolder "$image_stage" -format UDZO "$image_path"
(cd "$image_dir" && shasum -a 256 "$image_name" > "$image_name.sha256")
echo "Preview package: $image_path"
