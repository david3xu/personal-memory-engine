#!/usr/bin/env bash
# Bundle a checksum-verified official GitHub connector so sharing needs no developer installation.
set -euo pipefail
cd "$(dirname "$0")/../.."
gh_release=2.87.3
case "$(uname -m)" in arm64) gh_arch=arm64 ;; x86_64) gh_arch=amd64 ;; *) echo 'Unsupported macOS architecture' >&2; exit 1 ;; esac
gh_file="gh_${gh_release}_macOS_${gh_arch}.zip"
gh_cache="$PWD/target/vendor/github-cli/$gh_release/$gh_arch"
gh_resource="$PWD/web/owner/src-tauri/resources/github-cli"
mkdir -p "$gh_cache" "$gh_resource"
if [[ ! -f "$gh_cache/$gh_file" ]]; then
  curl --fail --location --proto '=https' --max-time 120 "https://github.com/cli/cli/releases/download/v$gh_release/$gh_file" -o "$gh_cache/$gh_file"
fi
curl --fail --location --proto '=https' --max-time 60 "https://github.com/cli/cli/releases/download/v$gh_release/gh_${gh_release}_checksums.txt" -o "$gh_cache/checksums.txt"
(cd "$gh_cache" && awk -v artifact="$gh_file" '$2 == artifact { print }' checksums.txt > selected.sha256 && test -s selected.sha256 && shasum -a 256 --check selected.sha256)
unzip -oq "$gh_cache/$gh_file" -d "$gh_cache/extracted"
ditto "$gh_cache/extracted/gh_${gh_release}_macOS_${gh_arch}/bin/gh" "$gh_resource/gh"
ditto "$gh_cache/extracted/gh_${gh_release}_macOS_${gh_arch}/LICENSE" "$gh_resource/LICENSE"
