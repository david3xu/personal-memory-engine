# Stage 01 implementation evidence

Updated 2026-10-07. Work is being implemented in Codex under D029. This record distinguishes compiled/tested behavior from the supplied brief's separate ChatGPT Work conversation requirement.

| Checkpoint | Evidence | Remaining |
| --- | --- | --- |
| 01.1 recording foundation | Rust contracts/operations, SQLite, real-process MCP tests; packaged helper tool discovery and restart checks pass; installed macOS app opens and displays a real user choice submitted through a direct stdio MCP client | Real ChatGPT Work conversation-to-card attribution check; computer controls cannot operate ChatGPT in this session |
| 01.2 history foundation | Atomic linked revisions, retry/concurrency/stale-reference checks, and desktop timeline implemented | Real worker revision walkthrough |
| 01.3 selected sharing | Approved design | Preview/projection, publication adapter, public URL and independent-reader checks |
| 01.4 installation/recovery | macOS Apple Silicon app bundle includes its helper; installation and launch on maintainer's Mac verified; plugin installed/enabled and launcher/tool discovery verified | Backup/restore, clean-machine installation, signing/notarization/distribution decisions |
| 01.5 integrated release | Draft PR #8, local checks, and Linux/macOS CI created | Complete acceptance, actual use/improvement, release and public demo evidence |

The first local user card records the explicit D029 implementation instruction. Rationale, rejected alternatives, and outside source evidence were omitted because none were supplied for that choice. Its worker metadata says **Codex (direct stdio MCP client)**. This is a real MCP-to-local-store-to-card capture, not a claim that ChatGPT Work called the tool.

Synthetic protocol tests use temporary isolated databases. Private runtime files and identifiers are excluded from Git; no local memory database is published.
