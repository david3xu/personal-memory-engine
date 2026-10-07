# Stage 01 implementation evidence

Updated 2026-10-08. Work is being implemented in Codex under D029. This record distinguishes compiled/tested behavior from the supplied brief's separate ChatGPT Work conversation requirement.

| Checkpoint | Evidence | Remaining |
| --- | --- | --- |
| 01.1 recording foundation | Rust contracts/operations, SQLite, real-process MCP tests; packaged helper tool discovery and restart checks pass; installed macOS app opens and displays a real user choice submitted through a direct stdio MCP client | Real ChatGPT Work conversation-to-card attribution check; computer controls cannot operate ChatGPT in this session |
| 01.2 history foundation | Atomic linked revisions, retry/concurrency/stale-reference checks, and desktop timeline implemented | Real worker revision walkthrough |
| 01.3 selected sharing | Approved design | Preview/projection, publication adapter, public URL and independent-reader checks |
| 01.4 installation/recovery | macOS Apple Silicon app bundle includes its helper; installation and launch on maintainer's Mac verified; bundled connector, graphical setup actions, fresh-test receipt and durable pause implemented; isolated catalog/launcher/recording tests pass | Backup/restore, real host link/install walkthrough, clean-machine installation, Developer ID signing and notarization |
| 01.5 integrated release | Draft PR #8, local checks, and Linux/macOS CI created | Complete acceptance, actual use/improvement, release and public demo evidence |

The first local user card records the explicit D029 implementation instruction. Rationale, rejected alternatives, and outside source evidence were omitted because none were supplied for that choice. Its worker metadata says **Codex (direct stdio MCP client)**. This is a real MCP-to-local-store-to-card capture, not a claim that ChatGPT Work called the tool.

Synthetic protocol tests use temporary isolated databases. Private runtime files and identifiers are excluded from Git; no local memory database is published.

## App-led onboarding checkpoint

D031 replaces the primary terminal setup with a bundled local catalog and graphical actions. Verification requires a fresh test request and successful synthetic record; reads, failures and previous retries cannot pass it. Pause persists across helper restarts and prevents tool reads/writes while owner card access remains available. Corrupt or future setup metadata fails closed. Mutable connection metadata is separate from append-only card history.

Local adapter tests cover the real stdio executable, installed launcher with spaces/apostrophes, link round-trips, resource completeness, restart/pause and fresh-test verification. The desktop build is separately checked for bundled resources; actual ChatGPT Work navigation and recording still require the user because computer controls cannot operate that app. There is no valid signing identity on the current Mac, so this distribution remains an unsigned preview.

The preview disk image is built without Finder layout automation after the default Tauri DMG bundler failed on the maintainer machine. It includes the complete app, an Applications shortcut and installation instructions. A checksum is generated alongside it.
